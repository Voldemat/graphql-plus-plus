import { RequestContext } from '../base.js'
import {
    AfterSerializationSubscriptionMiddleware,
    AfterSerializationSyncMiddleware,
    BeforeSerializationMiddleware
} from './serialization.js'
import {
    AfterParsingSubscriptionMiddleware,
    AfterParsingSyncMiddleware,
    BeforeParsingSubscriptionMiddleware,
    BeforeParsingSyncMiddleware
} from './parsing.js'

export interface ClientMiddlewaresConfig<
    TClientContext,
    TRequestContext extends RequestContext
> {
    beforeSerialization: BeforeSerializationMiddleware<
        TClientContext, TRequestContext
    >[]
    afterSerialization: {
        sync: AfterSerializationSyncMiddleware<
            TClientContext,
            TRequestContext
        >[],
        subscription: AfterSerializationSubscriptionMiddleware<
            TClientContext,
            TRequestContext
        >[],
    }
    beforeParsing: {
        sync: BeforeParsingSyncMiddleware<TClientContext, TRequestContext>[]
        subscription: BeforeParsingSubscriptionMiddleware<
            TClientContext,
            TRequestContext
        >[]
    }
    afterParsing: {
        sync: AfterParsingSyncMiddleware<TClientContext, TRequestContext>[],
        subscription: AfterParsingSubscriptionMiddleware<
            TClientContext,
            TRequestContext
        >[],
    }
}
