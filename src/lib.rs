// embedded-builder helpers for building embedded hardware interfaces
// Copyright 2018 Ryan Kurte

#![feature(used)]
#![feature(const_fn)]

#![no_std]

#[doc = "Helpers provide macros for the generation of accessors over Register objects"]
#[macro_use]
pub mod helpers;

#[doc = "Region provides types for type-safe memory mapping of memory regions"]
#[macro_use]
pub mod region;

#[doc = "Register provides a register type with chained building and modification"]
#[macro_use]
pub mod register;
