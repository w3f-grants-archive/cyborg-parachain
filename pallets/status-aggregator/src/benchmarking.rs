#![cfg(feature = "runtime-benchmarks")] // Ensures this code is only compiled when benchmarking is enabled

use super::*;
pub use cyborg_primitives::{oracle::ProcessStatus, worker::WorkerId};
use frame_benchmarking::v2::*;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn on_new_data() {
		// Constant defining the max number of feed values
		const MAX_FEED_VALUES: u32 = 12;

		#[block]
		{
			let max_limit = MAX_FEED_VALUES - 2;

			// Loop to generate and insert mock data for benchmarking
			for seed in 0..max_limit {
				// Generate a pseudo-random account ID using the `account` helper function
				let account_id: <T as frame_system::Config>::AccountId =
					account("benchmark_account", 0, seed);
				// Generate a pseudo-random worker ID
				let worker_id: WorkerId = (seed as u64) * 12345;
				// Create a ProcessStatus struct with random online/available status
				let process_status = ProcessStatus {
					online: seed % 2 == 0,
					available: seed % 3 == 0,
				};

				// Call the `on_new_data` function of the pallet with generated data
				Pallet::<T>::on_new_data(
					&account_id.clone(),
					&(account_id.clone(), worker_id),
					&process_status,
				);
			}
		}

		// Set up a test account and worker ID for validation after data insertion
		let test_account_id: <T as frame_system::Config>::AccountId =
			account("benchmark_account", 0, 1);
		let test_worker_id: WorkerId = (1 as u64) * 12345;

		// Assert that submission exists for the given account and worker ID in SubmittedPerPeriod
		assert!(
			SubmittedPerPeriod::<T>::get((
				test_account_id.clone(),
				(test_account_id.clone(), test_worker_id)
			)),
			"Submission not found"
		);

		// Assert that key exists in WorkerStatusEntriesPerPeriod for the test account and worker ID
		assert!(
			WorkerStatusEntriesPerPeriod::<T>::contains_key((test_account_id.clone(), test_worker_id)),
			"Entry key does not exists in WorkerStatusEntriesPerPeriod"
		);
	}

	// Defines the benchmark test suite, linking it to the pallet and mock runtime
	impl_benchmark_test_suite!(
		Pallet,
		crate::benchmarking::test::new_test_ext(),
		crate::mock::Test,
	);
}
