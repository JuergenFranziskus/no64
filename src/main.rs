use std::{
    collections::VecDeque,
    io::{self, stdout, Stdout},
    mem::take,
    thread::sleep,
    time::{Duration, Instant},
};

use cpu_mips3::{
    core::RawCore,
    vr4300::{SysAd, Vr4300},
};
use crossterm::event::{poll, Event, KeyCode, KeyEvent, KeyEventKind};
use no64::{memory::Memory, rsp::Rsp};
use terminal::Terminal;
use util::Word;

mod terminal;

fn main() -> anyhow::Result<()> {
    let app = App::new()?;
    app.run()?;
    Ok(())
}

struct App {
    running: bool,
    term: Terminal<Stdout>,
    cpu: Vr4300,
    mem: DummyBus,
    view_top: u64,
    line: String,
    errors: VecDeque<String>,
    delay: Duration,
    state: State,
}
impl App {
    fn new() -> anyhow::Result<Self> {
        let term = Terminal::init(stdout())?;
        let cpu = Vr4300::init();
        let mem = DummyBus::init();
        let view_top = 0;

        Ok(Self {
            running: true,
            term,
            cpu,
            mem,
            view_top,
            line: String::new(),
            errors: VecDeque::new(),
            delay: Duration::ZERO,
            state: State::Idle,
        })
    }

    fn run(mut self) -> anyhow::Result<()> {
        while self.running {
            self.handle_events()?;
            self.update()?;
            self.render()?;
            sleep(Duration::from_secs_f64(1.0 / 60.0));
        }

        Ok(())
    }
    fn handle_events(&mut self) -> anyhow::Result<()> {
        while Self::event_available()? {
            let ev = Self::next_event()?;
            self.handle_event(ev)?;
        }
        Ok(())
    }
    fn handle_event(&mut self, ev: Event) -> anyhow::Result<()> {
        match ev {
            Event::Key(key) => self.handle_key(key)?,
            _ => (),
        }
        Ok(())
    }
    fn handle_key(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        };

        match key.code {
            KeyCode::Char(c) => self.line.push(c),
            KeyCode::Backspace => {
                self.line.pop();
            }
            KeyCode::Esc => self.running = false,
            KeyCode::Enter => self.do_command()?,
            _ => (),
        }

        Ok(())
    }
    fn do_command(&mut self) -> anyhow::Result<()> {
        let command = take(&mut self.line);
        let words: Vec<_> = command.split_whitespace().collect();
        if words.len() == 0 {
            return Ok(());
        };
        let cmd = words[0];
        let args = &words[1..];
        match cmd {
            "run_for" | "rf" => self.do_run_for_command(args)?,
            "run_until" | "ru" => self.do_run_until_command(args)?,
            "delay" | "d" => self.do_delay_command(args)?,
            or => self.errors.push_back(format!("Unrecognized command: {or}")),
        }

        Ok(())
    }
    fn do_run_for_command(&mut self, args: &[&str]) -> anyhow::Result<()> {
        if args.len() != 1 {
            self.errors
                .push_back(format!("Wrong number of arguments, requires 1"));
            return Ok(());
        }
        let cycles_src = args[0];
        let Ok(cycles) = cycles_src.parse() else {
            self.errors.push_back(format!("Invalid argument"));
            return Ok(());
        };

        if self.state == State::Idle {
            self.state = State::Run(RunTarget::For(cycles), Instant::now());
        } else {
            self.errors
                .push_back(format!("Already executing another command"));
        }

        Ok(())
    }
    fn do_run_until_command(&mut self, args: &[&str]) -> anyhow::Result<()> {
        if args.len() != 1 {
            self.errors
                .push_back(format!("Wrong number of arguments, requires 1"));
            return Ok(());
        }

        let addr_src = args[0];
        let Ok(addr) = u64::from_str_radix(addr_src, 16) else {
            self.errors.push_back(format!("Invalid argument"));
            return Ok(());
        };

        if self.state == State::Idle {
            self.state = State::Run(RunTarget::Until(addr), Instant::now());
        } else {
            self.errors
                .push_back(format!("Already executing another command"));
        }

        Ok(())
    }
    fn do_delay_command(&mut self, args: &[&str]) -> anyhow::Result<()> {
        if args.len() != 1 {
            self.errors
                .push_back(format!("Wrong number of arguments, requires 1"));
            return Ok(());
        }
        let millis_src = args[0];
        let Ok(millis) = millis_src.parse() else {
            self.errors.push_back(format!("Invalid argument"));
            return Ok(());
        };

        self.delay = Duration::from_millis(millis);
        Ok(())
    }

    fn update(&mut self) -> anyhow::Result<()> {
        match self.state {
            State::Idle => (),
            State::Run(_, _) => self.update_command_run()?,
        }

        Ok(())
    }
    fn update_command_run(&mut self) -> anyhow::Result<()> {
        let State::Run(target, next) = &mut self.state else {
            unreachable!()
        };

        while Instant::now() >= *next {
            match target {
                RunTarget::For(cycles) => {
                    if *cycles == 0 {
                        self.state = State::Idle;
                        break;
                    } else {
                        *next += self.delay;
                    }
                    self.cpu.cycle(&mut self.mem);
                    *cycles -= 1;
                }
                RunTarget::Until(addr) => {
                    if *addr == self.cpu.program_counter() {
                        self.state = State::Idle;
                        break;
                    } else {
                        *next += self.delay;
                    }
                    self.cpu.cycle(&mut self.mem);
                }
            }
        }

        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        self.print_disassembly();
        self.print_registers();
        self.print_line();
        self.print_errors();
        self.term.print()?;
        Ok(())
    }
    fn print_disassembly(&mut self) {
        self.view_top = adjust_view_top(&self.cpu, self.view_top);

        for i in 0..50 {
            self.term.move_cursor(0, i);
            let addr = self.view_top + (i as u64) * 4;
            self.term.write(&format!("{addr:0>16x}: "));
            let pc = self.cpu.program_counter();
            if pc == addr {
                self.term.write("->");
            } else {
                self.term.write("  ");
            }

            let phys = self.cpu.translate_address_debug(addr);
            let word = phys.map(|p| self.mem.read_debug(p)).flatten();
            if let Some(word) = word {
                let word = word.to_u32(self.cpu.is_big_endian());
                let instr = cpu_mips3::instruction::Instr(word);
                let instr_str = format!("{instr}");
                self.term.write(&format!("{word:0>8x} {instr_str:<25}  "));

                let ls_addr = instr
                    .is_load_store()
                    .then(|| self.cpu.get_load_store_addr(instr));
                if let Some(Ok(addr)) = ls_addr {
                    self.term.write(&format!("=> {addr:0>16x} "));
                    if let Some(phys) = self.cpu.translate_address_debug(addr) {
                        self.term.write(&format!("= {phys:0>8x}"));
                    }
                }
            }
        }
    }
    fn print_registers(&mut self) {
        for i in 0..32 {
            self.term.move_cursor(100, i);
            let val = self.cpu.get_reg(i as u8).unwrap();
            self.term.write(&format!(
                "r{i:0>2}: {val:0>16x}  {val:>20}  {:>20}",
                val as i64
            ));
        }
    }
    fn print_line(&mut self) {
        self.term.move_cursor(0, 51);
        self.term.write(&self.line);
    }
    fn print_errors(&mut self) {
        while self.errors.len() > 10 {
            self.errors.pop_front();
        }

        for (i, err) in self.errors.iter().enumerate() {
            self.term.move_cursor(100, i + 40);
            self.term.write(&err);
        }
    }

    fn next_event() -> io::Result<Event> {
        crossterm::event::read()
    }
    fn event_available() -> io::Result<bool> {
        poll(Duration::ZERO)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    Idle,
    Run(RunTarget, Instant),
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum RunTarget {
    For(u64),
    Until(u64),
}

fn adjust_view_top(cpu: &Vr4300, view_top: u64) -> u64 {
    let pc = cpu.program_counter();

    const HI_BOUND: u64 = 40 * 4;

    if pc > view_top + HI_BOUND {
        pc - HI_BOUND
    } else if pc < view_top {
        pc
    } else {
        view_top
    }
}

pub struct DummyBus {
    rsp: Rsp,
    memory: Memory,
}
impl DummyBus {
    fn init() -> Self {
        Self {
            rsp: Rsp::init(),
            memory: Memory::init(),
        }
    }

    pub fn read_debug(&self, addr: u32) -> Option<Word> {
        self.memory.read_debug(addr)
    }
}
impl SysAd for DummyBus {
    fn read(&mut self, address: u32, size: cpu_mips3::vr4300::AccessSize) -> [Word; 2] {
        if let Some(data) = self.memory.read_for_cpu(address, size) {
            return data;
        }

        match address {
            RSP_REGS_FIRST..=RSP_REGS_LAST => self
                .rsp
                .read_for_cpu((address - RSP_REGS_FIRST) as usize, size),
            _ => [Word::zero(); 2],
        }
    }

    fn write(&mut self, address: u32, size: cpu_mips3::vr4300::AccessSize, data: [Word; 2]) {
        if self.memory.write_for_cpu(address, size, data) {
            return;
        };

        match address {
            RSP_REGS_FIRST..=RSP_REGS_LAST => {
                self.rsp
                    .write_for_cpu((address - RSP_REGS_FIRST) as usize, size, data)
            }
            _ => (),
        }
    }
}

const RSP_REGS_FIRST: u32 = 0x04040000;
const RSP_REGS_LAST: u32 = 0x040BFFFF;
