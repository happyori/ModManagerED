import type { Custom } from './Custom';
export type Data = {
    number: number;
    boolean: boolean;
    string: string;
    cstr: string;
    vec: Array<string>;
    array: number[];
    tuple: [number, string];
    custom_type: Custom;
    convoluted: any;
    retyped: string;
}
