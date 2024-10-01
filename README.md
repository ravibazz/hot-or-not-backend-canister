# HotOrNot Backend Canisters

## Deploying Canisters locally

To deploy canisters locally using dfx follow these steps.

### Step 1
Start dfx server 
```sh 
dfx start --background
```

### Step 2
Run the install script to build and deploy canisters. you can skip the test run by passing `-s` flag
```sh 
scripts/canisters/local_deploy/install_all_canisters.sh [-s]
```

**NOTE: This will only deploy one subnet-orchestrator (also called user-index in codebase) and will not deploy platform-orchsetrator. Platform-orchestrator needs to be deployed and tested separetly** 

## Upgrading locally deployed canisters
To upgrade locally deployed canisters. Run the following commands

### Step 1
Run the candid generator script to auto generate the candid files for the canisters.
```sh
scripts/candid_generator.sh
```

### Step 2
Build and upgrade the canisters deployed. You can pass `-s` flag to skip the tests
```sh
scripts/canisters/local_deploy/upgrade_all_canisters.sh [-s]
```

## Mainnet Deployment

### Mainnet Deployment Checks
These checks are important and should be strictly performed before raising any Pull request and ensure everything passes.

- checkout to the latest tag before the current build.
- deploy the canisters locally.
- Run the ic repl tests: `ic_repl_tests/all_tests.sh` (this would create some users locally and will add some posts for testing.)
- checkout to your branch
- run the upgrade process described above without skipping the tests.
- check if all the user canisters upgrade successfully and the posts are retained which were added by repl tests.


### Mainnet Deployment

The process of deploying to the mainnet is as follows:
- merge the Pull requests to the main branch
- create a semver tag for the release and push it.
- A github action would be triggered and raise the necessary proposals to upgrade the canisters

## Verifying builds

To get the hash for canisters:

- Get the canister IDs from [`canister_ids.json`](https://github.com/go-bazzinga/hot-or-not-backend-canister/blob/main/canister_ids.json).
- Get hash using the DFX SDK by running: `dfx canister info <canister-id> --network=ic`.

- The output of the above command should contain `Module hash` followed up with the hash value. Example output:

  ```
  $ > dfx canister info vyatz-hqaaa-aaaam-qauea-cai --network=ic

  Controllers: 7gaq2-4kttl-vtbt4-oo47w-igteo-cpk2k-57h3p-yioqe-wkawi-wz45g-jae
  wwyo5-vrahh-jwa74-3m6kj-jqbia-jbebm-7vtyd-uvqem-wk3zw-djpci-vqe
  Module hash: 0x98863747bb8b1366ae5e3c5721bfe08ce6b7480fe4c3864d4fec3d9827255480
  ```

To get the hash for canister deployment:

- Go to [Github actions deployment runs](https://github.com/go-bazzinga/hot-or-not-backend-canister/actions/workflows/webclient-deploy.yml)
- Open the latest succesful run. ([Click to see an example run](https://github.com/go-bazzinga/hot-or-not-backend-canister/actions/runs/4810296657))
- Go to any of the `Deploy all canisters` jobs. ([Click to see an example job](https://github.com/go-bazzinga/hot-or-not-backend-canister/actions/runs/4900015913/jobs/8750374252))
- Open one of the `Deploy <canister_name> canister` steps. You should find the `Module hash` in this step. This value should match the value you got locally. ([Click to see an example step](https://github.com/go-bazzinga/hot-or-not-backend-canister/actions/runs/4900015913/jobs/8750374252#step:8:16))

To check the status of the deployment

- check if the platform orchestrator performed the step to upgrade subnet canister with appropriate version: [Platform Orchestrator function](https://dashboard.internetcomputer.org/canister/74zq4-iqaaa-aaaam-ab53a-cai#get_subnet_last_upgrade_status)
- check the status of upgrade for individual canisters in subnet orchestrators and verify the version. Example for one of the subnet orchesrator: [Subnet Orchestrator function](https://dashboard.internetcomputer.org/canister/rimrc-piaaa-aaaao-aaljq-cai#get_index_details_last_upgrade_status)

---

