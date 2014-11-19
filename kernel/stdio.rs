use core::prelude::*;
use core::iter::range_step_inclusive;
use platform::vga::{Color, COLS, ROWS};
use platform::vga;

pub struct StdioWriter
{
	pub xpos: uint,
	pub ypos: uint,
	pub fg: Color,
	pub bg: Color
}

impl StdioWriter
{
	pub fn new() -> StdioWriter
	{
		StdioWriter
		{
			xpos: 0,
			ypos: 0,
			fg: Color::White,
			bg: Color::Black
		}
	}

	pub fn clear_screen(&mut self)
	{
		for y in range(0u, ROWS)
		{
			for x in range(0u, COLS)
			{
				vga::putc(x, y, 0);
				vga::setfg(x, y, self.fg);
				vga::setbg(x, y, self.bg);
			}
		}
		self.go_to(0, 0);
	}

	pub fn go_to(&mut self, x: uint, y: uint)
	{
		self.move_coords(x, y);
		self.set_cursor();
	}

	pub fn backspace(&mut self)
	{
		self.go_left();
		self.raw_print_char(' ' as u8);
		self.set_cursor();
	}

	pub fn tab(&mut self)
	{
		let x = self.xpos;
		for _ in range(0, 4 - (x % 4))
		{
			self.raw_print_char(' ' as u8);
			self.go_right();
		}
		self.set_cursor();
	}

	pub fn crlf(&mut self)
	{
		self.xpos = 0;
		self.ypos = if self.ypos == ROWS - 1 { 0 } else { self.ypos + 1 };
		self.set_cursor();
	}

	fn go_right(&mut self)
	{
		if self.xpos == COLS - 1
		{
			self.xpos = 0;
			self.ypos = (self.ypos + ROWS + 1) % ROWS;
		}
		else
		{
			self.xpos += 1;
		}
	}

	fn go_left(&mut self)
	{
		if self.xpos == 0
		{
			self.xpos = COLS - 1;
			self.ypos = (self.ypos + ROWS - 1) % ROWS;
		}
		else
		{
			self.xpos -= 1;
		}
	}

	fn move_coords(&mut self, x: uint, y: uint)
	{
		let mut newx = x;
		let mut newy = y;
		if newx >= COLS { newx = 0; newy += 1; }
		if newy >= ROWS { newy = 0; }
		self.xpos = newx;
		self.ypos = newy;
	}

	fn set_cursor(&self)
	{
		vga::move_cursor(self.xpos, self.ypos);
	}

	pub fn print_dec(&mut self, v: uint)
	{
		if v == 0
		{
			self.print_char('0');
			return;
		}

		let mut fac = 1;
		let mut nv = v;
		while fac <= v { fac *= 10; }
		fac /= 10;
		while fac > 0
		{
			let n = nv / fac;
			let c = n as u8 + '0' as u8;
			self.raw_print_char(c);
			self.go_right();
			nv -= n * fac;
			fac /= 10;
		}
		self.set_cursor();
	}

	pub fn print_bin(&mut self, v: uint, sz: uint)
	{
		self.print_screen("0b");

		for i in range_step_inclusive(sz as int - 1, 0, -1)
		{
			let c = match (v >> (i as uint)) & 0x1
			{
				0 => '0',
				_ => '1',
			} as u8;
			self.raw_print_char(c);
			self.go_right();
		}
		self.set_cursor();
	}

	pub fn print_hex(&mut self, v: uint, sz: uint)
	{
		self.print_screen("0x");

		for i in range_step_inclusive(sz as int - 4, 0i, -4)
		{
			let c = match (v >> (i as uint)) & 0xF
			{
				c if c <= 9 => c + '0' as uint,
				c => c + -10 + 'A' as uint,
			} as u8;
			self.raw_print_char(c);
			self.go_right();
		}
		self.set_cursor();
	}

	pub fn print_char(&mut self, value: char)
	{
		self.raw_print_char(value as u8);
		self.go_right();
		self.set_cursor();
	}

	fn raw_print_char(&self, value: u8)
	{
		vga::putc(self.xpos, self.ypos, value as u8);
		vga::setfg(self.xpos, self.ypos, self.fg);
		vga::setbg(self.xpos, self.ypos, self.bg);
	}

	pub fn print_screen(&mut self, value: &str)
	{
		for c in value.bytes()
		{
			self.raw_print_char(c);
			self.go_right();
		}
		self.set_cursor();
	}
}

impl ::core::fmt::FormatWriter for StdioWriter
{
	fn write(&mut self, bytes: &[u8]) -> ::core::fmt::Result
	{
		for b in bytes.iter()
		{
			self.raw_print_char(*b);
			self.go_right();
		}
		self.set_cursor();
		Ok(())
	}
}
