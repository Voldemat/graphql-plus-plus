import { OperationLoadingState } from './useOperation.jsx';

export const loadingState =
    Object.freeze({ state: 'loading' } as const) satisfies OperationLoadingState

