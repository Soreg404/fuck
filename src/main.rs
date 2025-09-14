use std::io::{stdout, Write};
use std::os::windows::process::CommandExt;
use std::process::ExitCode;

fn main() -> ExitCode {
	let line = match get_command_to_correct() {
		Some(v) => v,
		None => return ExitCode::from(0u8)
	};

	let correct = correct_me(&line);

	let status = match correct {
		None => {
			println!("{:?}", line);
			println!("can't do sh!t here, mate!");
			0u8
		}
		Some(s) => {
			println!("correcting to \x1b[96m{s:?}\x1b[0m");
			println!();
			let status = std::process::Command::new("cmd")
				.arg("/c")
				.raw_arg(s)
				.status()
				.unwrap();
			status.code().unwrap_or(0) as u8
		}
	};


	if status != 0u8 {
		print!("\x1b[91m{:?}\x1b[0m", status);
	} else {
		print!("\x1b[96m{:?}\x1b[0m", status);
	}
	_ = stdout().flush();

	ExitCode::from(status)
}

fn get_command_to_correct() -> Option<String> {
	let output = std::process::Command::new("doskey")
		.arg("/history")
		.output()
		.unwrap();

	let output = String::from_utf8_lossy(output.stdout.as_slice());
	let output = output.trim();

	let output = output.rfind('\n').map(|index| {
		output[..index].trim()
	})
		.unwrap_or_default();

	if output.is_empty() {
		println!("nothing to correct??");
		return None;
	}

	let line = match output.rfind('\n') {
		Some(index) => &output[(index + 1)..],
		None => &output,
	};

	Some(line.to_string())
}

fn correct_me(line: &str) -> Option<String> {
	let first_word = line.split_whitespace().next().unwrap();

	match first_word {
		"gut" => {
			println!("gut -> git");
			let correct = format!("git {}", &line[4..]);
			Some(correct)
		}
		_ => None
	}
}
