export type Credential = {
    url: string,
    username: string,
    password: string
}

export type KeeperCredential = {
    hash: string,
    credential: Credential
}