# Introduction

The userexample folder contains a cargo binary. This demos how the generated Rust code in this
repo can be used to upload data to a GCP bucket.

# How to run the demo

1. Create ADC credentials `$ gcloud auth application-default login`
1. Set up bucket name `$ export DEMO_BUCKET=<YOUR_BUCKET>`
    - Create the bucket if it does not yet exist `$ gcloud storage buckets create gs://$DEMO_BUCKET`
1. Set up the absolute file path for the text document the demo should upload
   `$ export DEMO_DATA_PATH=$(realpath <YOUR_FILE>)`.
1. Execute `$ cargo run` in the `userexample` directory to run the demo.
