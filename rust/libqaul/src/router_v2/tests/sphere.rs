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

/// `None` is not a §4.1 transport — it is the enum's unknown-value slot,
/// and it is now classified deliberately rather than by a catch-all arm.
/// Local keeps a stray value inside the membrane; classifying it Internet
/// would make it a path for village state to leak outward.
#[test]
fn sphere_of_none_is_deliberately_local() {
    assert_eq!(Sphere::of(ConnectionModule::None), Sphere::Local);
}

/// §2.3 and §4.1 require every transport to declare a sphere. This pins the
/// declaration for all six variants in one place: if a variant is added to
/// [`ConnectionModule`], `Sphere::of` fails to compile and this test is
/// where the new transport's sphere gets recorded.
#[test]
fn every_connection_module_declares_a_sphere() {
    let declared = [
        (ConnectionModule::Local, Sphere::Local),
        (ConnectionModule::Lan, Sphere::Local),
        (ConnectionModule::Internet, Sphere::Internet),
        (ConnectionModule::Ble1m, Sphere::Local),
        (ConnectionModule::BleCoded, Sphere::Local),
        (ConnectionModule::None, Sphere::Local),
    ];

    for (module, expected) in declared {
        assert_eq!(Sphere::of(module), expected, "sphere of {module:?}");
    }
}
