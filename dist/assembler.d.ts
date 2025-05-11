/* tslint:disable */
/* eslint-disable */
export function gen_encoding(code: string, state: MachineState): void;
export function step(state: MachineState): RegisterTransfer[];
export function rt_get_string(rt: RegisterTransfer): string;
export interface MachineState {
    memory: number[];
    reg_stack: RegisterStack;
    handling_right: boolean;
    right_fetch: boolean;
}

export interface RegisterTransferList {
    transfer: RegisterTransfer[];
}

export type Register = "AC" | "MQ" | "MBR" | "IBR" | "IR" | "PC" | "MAR";

export type Addressing = { Register: Register } | { Unary: [Register, UnaryOperation] } | { MixedReg: [Register, Register, BinaryOperation] } | { MixedConst: [Register, number, BinaryOperation] } | "Memory" | { Constant: number };

export type UnaryOperation = "BitFlip" | "LeftShift" | "RightShift";

export type BinaryOperation = "Addition" | "Subtraction" | "Multiplication" | "Remainder" | "Division";

export type Amount = "Full" | { Range: { start: number; end: number } };

export interface Operand {
    operand_type: Addressing;
    amount: Amount;
}

export interface RegisterTransfer {
    from: Operand;
    to: Operand;
}

export class MachineState {
  free(): void;
  get_clone(): MachineState;
  constructor();
  memory: BigInt64Array;
  readonly get_reg_stack: RegisterStack;
}
export class RegisterStack {
  private constructor();
  free(): void;
  ac: bigint;
  mq: bigint;
  pc: bigint;
  readonly get_ac: bigint;
  readonly get_mq: bigint;
  readonly get_pc: bigint;
}
export class RegisterTransferList {
  private constructor();
  free(): void;
  readonly get_transfer: RegisterTransfer[];
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_machinestate_free: (a: number, b: number) => void;
  readonly machinestate_memory: (a: number) => [number, number];
  readonly machinestate_set_memory: (a: number, b: number, c: number) => void;
  readonly machinestate_get_clone: (a: number) => number;
  readonly __wbg_registertransferlist_free: (a: number, b: number) => void;
  readonly registertransferlist_get_transfer: (a: number) => [number, number];
  readonly machinestate_new: () => number;
  readonly machinestate_get_reg_stack: (a: number) => number;
  readonly __wbg_registerstack_free: (a: number, b: number) => void;
  readonly __wbg_get_registerstack_ac: (a: number) => bigint;
  readonly __wbg_set_registerstack_ac: (a: number, b: bigint) => void;
  readonly __wbg_get_registerstack_mq: (a: number) => bigint;
  readonly __wbg_set_registerstack_mq: (a: number, b: bigint) => void;
  readonly __wbg_get_registerstack_pc: (a: number) => bigint;
  readonly __wbg_set_registerstack_pc: (a: number, b: bigint) => void;
  readonly registerstack_get_ac: (a: number) => bigint;
  readonly registerstack_get_mq: (a: number) => bigint;
  readonly registerstack_get_pc: (a: number) => bigint;
  readonly gen_encoding: (a: number, b: number, c: number) => void;
  readonly step: (a: number) => [number, number];
  readonly rt_get_string: (a: any) => [number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
