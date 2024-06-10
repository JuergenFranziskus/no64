use std::{
    collections::VecDeque,
    io::{self, stdout, Stdout, Write},
    mem::{replace, take},
    thread::sleep,
    time::{Duration, Instant},
};

use cpu_mips3::{
    core::{MipsResult, RawCore},
    instruction::{print_instr, Instr, Reg},
    vr4300::{SysAd, Vr4300, WriteSize},
    word::Word,
};
use crossterm::event::{poll, Event, KeyCode, KeyEvent, KeyEventKind};
use no64::{console::Console, pif_nus::PifNus, rsp::Rsp};
use terminal::Terminal;

mod terminal;

fn main() -> anyhow::Result<()> {
    let app = App::new()?;
    app.run()?;
    Ok(())
}

struct App {
    console: Console,
    running: bool,
    term: Terminal<Stdout>,
    line: String,
    errors: VecDeque<String>,
    state: State,
    delay: Duration,
}
impl App {
    fn new() -> anyhow::Result<Self> {
        let term = Terminal::init(stdout())?;

        Ok(Self {
            console: Console::init(),
            running: true,
            term,
            line: String::new(),
            errors: VecDeque::new(),
            state: State::Idle,
            delay: Duration::ZERO,
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
            "delay" | "d" => self.do_delay_command(args)?,
            "run_for" | "rf" => self.do_run_for_command(args)?,
            or => self.errors.push_back(format!("Unrecognized command: {or}")),
        }

        Ok(())
    }
    fn do_delay_command(&mut self, args: &[&str]) -> anyhow::Result<()> {
        if args.len() != 1 {
            self.errors.push_back("Wrong amount of arguments".into());
            return Ok(());
        }

        let Ok(millis) = args[0].parse() else {
            self.errors.push_back("Could not parse argument".into());
            return Ok(());
        };

        self.delay = Duration::from_millis(millis);
        Ok(())
    }
    fn do_run_for_command(&mut self, args: &[&str]) -> anyhow::Result<()> {
        if args.len() != 1 {
            self.errors.push_back("Wrong amount of arguments".into());
            return Ok(());
        }

        let Ok(cycles) = args[0].parse() else {
            self.errors.push_back("Could not parse argument".into());
            return Ok(());
        };

        if self.state != State::Idle {
            self.errors.push_back("Already executing different command".into());
            return Ok(());
        }

        self.state = State::RunFor(cycles, Instant::now());
        Ok(())
    }

    fn update(&mut self) -> anyhow::Result<()> {
        let state = replace(&mut self.state, State::Idle);
        match state {
            State::Idle => (),
            State::RunFor(cycles, next) => self.update_run_for(cycles, next)?,
            State::RunUntil(addr, next) => self.update_run_until(addr, next)?,
        }

        Ok(())
    }
    fn update_run_for(&mut self, mut cycles: usize, mut next: Instant) -> anyhow::Result<()> {
        while cycles != 0 {
            if Instant::now() < next {
                self.state = State::RunFor(cycles, next);
                break;
            }
            self.step_emulator()?;
            next = next + self.delay;
            cycles -= 1;
        }
        if cycles != 0 {
            self.state = State::RunFor(cycles, next);
        }


        Ok(())
    }
    fn update_run_until(&mut self, addr: u64, next: Instant) -> anyhow::Result<()> {
        todo!()
    }

    fn step_emulator(&mut self) -> anyhow::Result<()> {
        let (cpu, mut bus) = self.console.cpu_and_bus();
        let res = cpu.step_forward(&mut bus);

        if let Err(err) = res {
            self.render()?;
            println!("Could not step emulator forward");
            println!("{}", err.at());

            return Err(anyhow::Error::msg("All fucked up"));
        }

        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        self.print_disassembly()?;
        self.print_registers()?;
        self.print_line();
        self.print_errors();
        self.term.print()?;
        Ok(())
    }
    fn print_disassembly(&mut self) -> io::Result<()> {
        let pc = self.console.cpu.program_counter();
        let first = pc - 15 * 4;
        let last = pc + 14 * 4;
        let mut previous = None;

        for (i, addr) in (first..last).step_by(4).enumerate() {
            let prev = previous.take();
            self.term.move_cursor(0, i);
            write!(self.term, "{addr:0>16x}")?;
            let Some(phys) = self.console.cpu.translate_address_debug(addr) else { continue };
            let c = if phys.cached { "C" } else { " " };
            let arrow = if pc == addr { "->" } else { "  " };
            write!(self.term, " {c} {:0>8x} {arrow}", phys.addr)?;
            let Some(word) = self.console.read_debug(phys.addr) else { continue };
            let val = word.to_u32(self.console.cpu.is_big_endian());
            write!(self.term, "{val:0>8x} ")?;
            let instr = Instr(val);
            previous = Some(instr);

            print_instr(&mut self.term, instr)?;
        }
        Ok(())
    }
    fn print_registers(&mut self) -> io::Result<()> {
        for i in 0..32 {
            let r = Reg(i);
            self.term.move_cursor(100, i as usize);
            let val = self.console.cpu.get_reg_i64(r).unwrap_or(0);
            cpu_mips3::instruction::print_gp_reg(&mut self.term, r)?;
            write!(self.term, " = 0x{val:x}")?;
        }

        Ok(())
    }
    fn print_line(&mut self) {
        self.term.move_cursor(0, 51);
        self.term.write_text(&self.line);
    }
    fn print_errors(&mut self) {
        while self.errors.len() > 10 {
            self.errors.pop_front();
        }

        for (i, err) in self.errors.iter().enumerate() {
            self.term.move_cursor(100, i + 40);
            self.term.write_text(&err);
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
    RunFor(usize, Instant),
    RunUntil(u64, Instant),
}
