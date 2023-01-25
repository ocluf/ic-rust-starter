# ic-rust-starter

I often find myself making the same edits to the `dfx new -type rust` starter template. This alternative starter template contains the following:

- Different directory structure without a frontend 
- Automatic Candid file generation if you run cargo test 
- A global State object using the thread_local pattern which is saved in stable memory across updates
- An example of a `update` and `query` call
- Updated release profile based on this great [blog post about Effective Rust canisters](https://mmapped.blog/posts/01-effective-rust-canisters.html) 
- A devcontainer configuration so you can install the entire needed toolchain with a single click in vscode based on this [tutorial](https://medium.com/@ilbert/rust-react-typescript-internet-identity-on-the-internet-computer-35331ae2a4be) (needs docker)

> :warning: Sadly the devcontainer does not work on ARM macs. Please open an issue if you experience issues on other platforms as I couldn't test those

### Getting started 

1. clone the repo and open in vscode 
2. If you don't have an M1/M2 mac click the tooltip that shows up to start the devcontainer
3. in the devcontainer terminal run `dfx start --background" 
4. run `dfx deploy`

If you do have an M1 of M2 mac 

1. clone the repo and open in vscode and do not click the popup
2. remove the .devcontainer directory
3. make sure you have [rust toolchain](https://www.rust-lang.org/tools/install) installed 
4. run `dfx start --background" 
5. run `dfx deploy`

### Changing the project name 


To give the project your desired project name just run a find and replace on all instances of `starter` to `your_project_name` and change the name of the `starter` directory and the `starter.did` file to `your_project_name` and `your_project_name.did` as well. 

---

Final tip: You can add additional fields to your structs accross upgrades if you wrap the new field values with the `Option` type. Otherwise you will get an error in the post_upgrade hook and the upgrade will fail. 


Feel free to open up an issue if you'd like an example of different concepts as well.

