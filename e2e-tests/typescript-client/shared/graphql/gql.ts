import { beforeAll } from "bun:test"
import { type Context, createExecutor } from "./executor";
import { createSdk } from "./generated/gql-client";
import { createObjectProxy } from "@shared/fixtures/proxy";

export type Sdk = ReturnType<typeof createSdk>

export function useSdk(
    initContext: Context = { accessToken: null, refreshToken: null }
): Sdk {
    let sdk: Sdk

    beforeAll(() => {
        sdk = createSdk(createExecutor(initContext))
    })

    return createObjectProxy(() => sdk)
}
