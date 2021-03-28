
// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


#[cfg(target_endian = "big")]
macro_rules! byteread {
	(64, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr) => {
		u64::from_le_bytes([$a, $b, $c, $d, $e, $f, $g, $h])
	};

	(64, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => {
		u64::from_le_bytes([$a, $b, $c, $d, $e, $f, $g,  0])
	};

	(64, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
		u64::from_le_bytes([$a, $b, $c, $d, $e, $f,  0,  0])
	};

	(64, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
		u64::from_le_bytes([$a, $b, $c, $d, $e,  0,  0,  0])
	};

	(64, $a:expr, $b:expr, $c:expr, $d:expr) => {
		u64::from_le_bytes([$a, $b, $c, $d,  0,  0,  0,  0])
	};

	(64, $a:expr, $b:expr, $c:expr) => {
		u64::from_le_bytes([$a, $b, $c,  0,  0,  0,  0,  0])
	};

	(64, $a:expr, $b:expr) => {
		u64::from_le_bytes([$a, $b,  0,  0,  0,  0,  0,  0])
	};

	(64, $a:expr) => {
		u64::from_le_bytes([$a,  0,  0,  0,  0,  0,  0,  0])
	};

	(32, $a:expr, $b:expr, $c:expr, $d:expr) => { u32::from_be_bytes([$a, $b, $c, $d]) };
	(32, $a:expr, $b:expr, $c:expr)          => { u32::from_be_bytes([$a, $b, $c,  0]) };
	(32, $a:expr, $b:expr)                   => { u32::from_be_bytes([$a, $b,  0,  0]) };
	(32, $a:expr)                            => { u32::from_be_bytes([$a,  0,  0,  0]) };

	(16, $a:expr, $b:expr)                   => { u16::from_be_bytes([$a, $b]) };
	(16, $a:expr)                            => { u16::from_be_bytes([$a,  0]) };
}

#[macro_export]
#[cfg(target_endian = "little")]
macro_rules! byteread {
	(64, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr) => {
		u64::from_le_bytes([$a, $b, $c, $d, $e, $f, $g, $h])
	};

	(64, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => {
		u64::from_le_bytes([ 0, $a, $b, $c, $d, $e, $f, $g])
	};

	(64, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
		u64::from_le_bytes([ 0,  0, $a, $b, $c, $d, $e, $f])
	};

	(64, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
		u64::from_le_bytes([ 0,  0,  0, $a, $b, $c, $d, $e])
	};

	(64, $a:expr, $b:expr, $c:expr, $d:expr) => {
		u64::from_le_bytes([ 0,  0,  0,  0, $a, $b, $c, $d])
	};

	(64, $a:expr, $b:expr, $c:expr) => {
		u64::from_le_bytes([ 0,  0,  0,  0,  0, $a, $b, $c])
	};

	(64, $a:expr, $b:expr) => {
		u64::from_le_bytes([ 0,  0,  0,  0,  0,  0, $a, $b])
	};

	(64, $a:expr) => {
		u64::from_le_bytes([ 0,  0,  0,  0,  0,  0,  0, $a])
	};

	(32, $a:expr, $b:expr, $c:expr, $d:expr) => { u32::from_le_bytes([$a, $b, $c, $d]) };
	(32, $a:expr, $b:expr, $c:expr)          => { u32::from_le_bytes([ 0, $a, $b, $c]) };
	(32, $a:expr, $b:expr)                   => { u32::from_le_bytes([ 0,  0, $a, $b]) };
	(32, $a:expr)                            => { u32::from_le_bytes([ 0,  0,  0, $a]) };

	(16, $a:expr, $b:expr)                   => { u16::from_le_bytes([$a, $b]) };
	(16, $a:expr)                            => { u16::from_le_bytes([ 0, $a]) };
}