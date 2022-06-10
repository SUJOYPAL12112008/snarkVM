// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

impl<N: Network> ToBits for Address<N> {
    /// Outputs the little-endian bit representation of `self.to_x_coordinate()` *without* trailing zeros.
    fn to_bits_le(&self) -> Vec<bool> {
        self.address.to_x_coordinate().to_bits_le()
    }

    /// Outputs the big-endian bit representation of `self.to_x_coordinate()` *without* leading zeros.
    fn to_bits_be(&self) -> Vec<bool> {
        self.address.to_x_coordinate().to_bits_be()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network::Testnet3;

    type CurrentNetwork = Testnet3;

    const ITERATIONS: u64 = 10_000;

    #[test]
    fn test_to_bits_le() {
        for _ in 0..ITERATIONS {
            // Sample a random value.
            let group: Group<CurrentNetwork> = Uniform::rand(&mut test_rng());

            let address = Address::new(group);
            let candidate = address.to_bits_le();
            assert_eq!(Address::<CurrentNetwork>::size_in_bits(), candidate.len());

            for (expected, candidate) in (*address).to_affine().to_x_coordinate().to_bits_le().iter().zip_eq(&candidate)
            {
                assert_eq!(expected, candidate);
            }
        }
    }

    #[test]
    fn test_to_bits_be() {
        for _ in 0..ITERATIONS {
            // Sample a random value.
            let group: Group<CurrentNetwork> = Uniform::rand(&mut test_rng());

            let address = Address::new(group);
            let candidate = address.to_bits_be();
            assert_eq!(Address::<CurrentNetwork>::size_in_bits(), candidate.len());

            for (expected, candidate) in (*address).to_affine().to_x_coordinate().to_bits_be().iter().zip_eq(&candidate)
            {
                assert_eq!(expected, candidate);
            }
        }
    }
}
