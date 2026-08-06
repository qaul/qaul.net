// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Transport-to-sphere classification (spec §2.3).

use crate::router_v2::*;

#[test]
fn sphere_of_lan_is_local() {
    assert_eq!(Sphere::of(ConnectionModule::Lan), Sphere::Local);
}

#[test]
fn sphere_of_ble1m_is_local() {
    assert_eq!(Sphere::of(ConnectionModule::Ble1m), Sphere::Local);
}

#[test]
fn sphere_of_ble_coded_is_local() {
    assert_eq!(Sphere::of(ConnectionModule::BleCoded), Sphere::Local);
}

#[test]
fn sphere_of_internet_is_internet() {
    assert_eq!(Sphere::of(ConnectionModule::Internet), Sphere::Internet);
}

#[test]
fn sphere_of_self_is_local() {
    // ConnectionModule::Local refers to this node itself, which is
    // part of its own Local sphere by definition
    assert_eq!(Sphere::of(ConnectionModule::Local), Sphere::Local);
}

#[test]
fn sphere_of_none_currently_falls_through_to_local() {
    assert_eq!(Sphere::of(ConnectionModule::None), Sphere::Local);
}
