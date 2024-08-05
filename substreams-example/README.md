# Running
1. Edit [package.json](./package.json) to set `--node` and `--ipfs` to point at the graphnode
2. Build the spkg
    ```shell
    pnpm run build
    ```
3. Create the subgraph
    ```shell
    pnpm run create
    ```
4. Deploy the subgraph
    ```shell
    pnpm run deploy
    ```