import { Credential } from "../types";

export default class CredentialStorage {
    private book: Map<String, Credential> = new Map();
    add(k:String, c: Credential) {
        this.book.set(k, c);
    }
    bulk() {
        return this.book;
    }
}