// Copyright 2019-2021 Manta Network.
// This file is part of manta-crypto.
//
// manta-crypto is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// manta-crypto is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with manta-crypto.  If not, see <http://www.gnu.org/licenses/>.
#![cfg_attr(not(feature = "std"), no_std)]

mod checksum;
mod commitment;
mod constants;
mod ecies;
mod merkle_tree;
mod param;
mod serdes;

#[cfg(test)]
mod tests;

pub use checksum::*;
pub use commitment::*;
pub use constants::*;
pub use ecies::*;
pub use merkle_tree::*;
pub use param::*;
pub use serdes::*;

pub struct MantaCrypto;
