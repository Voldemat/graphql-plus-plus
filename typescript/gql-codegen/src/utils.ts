export function keys<T>(obj: object): (keyof T)[] {
    return Object.keys(obj) as (keyof T)[]
}

type Entries<T> = {
    [K in keyof T]-?: [K, T[K]];
}[keyof T][];
export function entries<T>(obj: object): Entries<T> {
    return Object.entries(obj) as Entries<T>
}

export function fromEntries <
    const T extends ReadonlyArray<readonly [PropertyKey, unknown]>
>(eList: T): { [K in T[number]as K[0]]: K[1] } {
    return Object.fromEntries(eList) as { [K in T[number]as K[0]]: K[1] };
};
