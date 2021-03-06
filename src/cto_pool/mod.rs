// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use IsNotNull;
use self::arc::CtoArc;
use self::collections::CtoVec;
use self::synchronisation::CtoParkingLotMutexLock;
use self::synchronisation::CtoParkingLotReadWriteLock;
use self::synchronisation::CtoParkingLotReentrantMutexLock;
use self::string::CtoString;
use self::boxed::CtoBox;
use self::rc::CtoRc;
use ::libc::c_void;
use ::libc::mode_t;
use ::libc::size_t;
use ::std::borrow::Borrow;
use ::std::borrow::BorrowMut;
use ::std::cmp::min;
use ::std::cmp::Ordering;
use ::std::error;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Pointer;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::heap::Alloc;
use ::std::heap::AllocErr;
use ::std::heap::CannotReallocInPlace;
use ::std::heap::Excess;
use ::std::heap::Layout;
use ::std::marker::PhantomData;
use ::std::mem::align_of;
use ::std::mem::size_of;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::drop_in_place;
use ::std::ptr::NonNull;
use ::std::ptr::write;
use ::std::path::Path;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering::SeqCst;

/// An Arc like that in regular Rust's stdlib.
pub mod arc;

/// A block_allocator
pub mod block_allocator;

/// A Box like that in regular Rust's stdlib.
pub mod boxed;

/// Collections
pub mod collections;

/// A fetch-and-add array queue (`FAAArrayQueue`) by Pedro Ramalhete & Andreia Correia of Concurrency Freaks
/// See https://github.com/pramalhe/ConcurrencyFreaks/blob/master/CPP/queues/array/FAAArrayQueue.hpp and the Concurrency Freaks blog.
pub mod fetch_and_add_array_queue;

/// A non-blocking free list that is persistent.
/// Start with `CtoFreeListArc`.
pub mod free_list;

/// Extensions and wrapper to make use of parking lot's excellent synchronisation primitives.
pub mod synchronisation;

/// A Rc like that in regular Rust's stdlib.
pub mod rc;

/// A String like that in regular Rust's stdlib.
pub mod string;


const PMEMCTO_MAX_LAYOUT: size_t = 1024;


include!("Allocator.rs");
include!("CtoPool.rs");
include!("CtoPoolAlloc.rs");
include!("CtoPoolAllocationError.rs");
include!("CtoPoolArcInner.rs");
include!("CtoPoolArc.rs");
include!("CtoPoolOpenError.rs");
include!("CtoPoolPathExt.rs");
include!("CtoSafe.rs");
include!("Persistence.rs");
include!("PersistentMemoryWrapper.rs");
include!("PmdkError.rs");
include!("PMEMctopool.rs");
include!("PMEMctopoolExt.rs");
