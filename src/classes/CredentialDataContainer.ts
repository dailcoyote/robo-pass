import { Credential } from "../types";

export default class CredentialDataContainer {
    private vector: Array<Credential> = new Array();
    add(c: Credential) {
        this.vector.push(c);
    }
    list() {
        return this.vector;
    }
}