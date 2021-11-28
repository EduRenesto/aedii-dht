#[derive(Copy, Clone)]
pub struct InfoHash([u32; 5]);

impl InfoHash {
    pub fn dist(&self, other: &InfoHash) -> InfoHash {
	InfoHash(
	    self
		.0
		.zip(other.0)
		.map(|(x, y)| x ^ y)
	)
    }

    pub fn random() -> InfoHash {
	InfoHash([
	    rand::random::<u32>(),
	    rand::random::<u32>(),
	    rand::random::<u32>(),
	    rand::random::<u32>(),
	    rand::random::<u32>(),
	])
    }

    pub const fn zero() -> InfoHash {
	InfoHash([0; 5])
    }
}

impl std::cmp::PartialEq for InfoHash {
    fn eq(&self, other: &Self) -> bool {
	self
	    .0
	    .iter()
	    .zip(other.0.iter())
	    .map(|(a, b)| a == b)
	    .all(|x| x)
    }
}

impl std::fmt::Display for InfoHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	for lane in self.0.iter().rev() {
	    write!(f, "{:4x}", lane)?;
	}

	Ok(())
    }
}

impl std::fmt::Debug for InfoHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dist_with_zero_is_self() {
	let a = InfoHash::random();
	let zero = InfoHash::zero();

	let dist = a.dist(&zero);

	assert_eq!(a, dist);
    }

    #[test]
    fn dist_with_self_is_zero() {
	let a = InfoHash::random();
	let dist = a.dist(&a);

	let zero = InfoHash::zero();

	assert_eq!(dist, zero);
    }
}
