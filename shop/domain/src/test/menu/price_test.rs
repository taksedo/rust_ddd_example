#![allow(non_snake_case)]

use std::str::FromStr;

use bigdecimal::BigDecimal;
use common::types::common::count::Count;
use rstest::rstest;

use crate::main::menu::value_objects::price::{CreatePriceError, Price};
