declare module '*/tambo_wasm.js' {
  export default function init(module?: URL | string): Promise<void>
  export function generate_pdf(json: string, template: string): Uint8Array
  export function generate_standalone_typ(json: string, template: string): string
}
