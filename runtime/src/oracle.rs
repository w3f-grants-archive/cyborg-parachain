use frame_support::{pallet_prelude::*, traits::Time};
use scale_info::prelude::string::String;
use sp_std::vec::Vec;
use orml_traits;
use orml_oracle::Config;

/// Interface for fetching metrics and Logs.
///
/// **NOTE:** This is just a temporary interface, and will be replaced with a proper oracle which will 
/// provide metrics and logs data of a connected cluster
pub trait MetricsAndLogs {
    /// get metrics
    fn get_metrics() -> StringAPI;
}

pub type StringAPI = String;

#[derive(Encode, Decode, MaxEncodedLen, Clone, Copy, Debug, Ord, PartialOrd, PartialEq, Eq, TypeInfo)]
pub struct ProcessStatus {
    TaskCompleted: bool,
    // TaskResultHash: Option<H256>,
}

#[derive(Encode, Decode, MaxEncodedLen, Clone, Copy, Debug, PartialEq, Eq, TypeInfo)]
pub enum MachineId {
    Id(u64),
}

#[derive(Encode, Decode, MaxEncodedLen, Clone, Copy, Debug, PartialEq, Eq, TypeInfo)]
pub enum ProcessId {
    Process(u64, MachineId),
}

type TimestampedValue<T, I = ()> = orml_oracle::TimestampedValue<
    ProcessStatus,
    <<T as orml_oracle::Config<I>>::Time as Time>::Moment,
>;

/// A dummy implementation of `CombineData` trait that does nothing.
pub struct DummyCombineData<T, I = ()>(PhantomData<(T, I)>);
impl<T: orml_oracle::Config<I>, I> orml_traits::CombineData<ProcessId, TimestampedValue<T, I>>
    for DummyCombineData<T, I> where <T as Config<I>>::Time: frame_support::traits::Time
{
    fn combine_data(
        _key: &ProcessId,
        _values: Vec<TimestampedValue<T, I>>,
        _prev_value: Option<TimestampedValue<T, I>>,
    ) -> Option<TimestampedValue<T, I>> {
        None
    }
}