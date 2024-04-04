export type Credential = {
    url: string,
    username: string,
    password: string
}

export type KeeperCredential = {
    keeper_id: string,
    privacy: Credential
}