// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<E: Environment> Distribution<StringType<E>> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StringType<E> {
        // Sample a random number up to 1/4th of the maximum bytes.
        let num_bytes = rng.gen_range(1..(E::MAX_STRING_BYTES / 4) as usize);
        // Sample a random string.
        StringType::new(&rng.sample_iter(&Alphanumeric).take(num_bytes).map(char::from).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    type CurrentEnvironment = Console;

    const ITERATIONS: usize = 100;

    #[test]
    fn test_random() {
        // Initialize a set to store all seen random elements.
        let mut set = HashSet::with_capacity(ITERATIONS);

        let mut rng = TestRng::default();

        // Note: This test technically has a `(1 + 2 + ... + ITERATIONS) / MODULUS` probability of being flaky.
        for _ in 0..ITERATIONS {
            // Sample a random value.
            let string: StringType<CurrentEnvironment> = Uniform::rand(&mut rng);
            assert!(!set.contains(&string), "{}", string);

            // Add the new random value to the set.
            set.insert(string);
        }
    }
}
