/**
 * Save game or pack operation in progress (S-6 "Gleichzeitigkeit"): a
 * second one is refused with `busy`; a due autosave waits for its end.
 * Module "Plattform", step 3.10.
 */
import { fail } from "./platform-error.js";

export class OperationLock {
  private current: string | undefined;

  /** True while an autosave is due and waits for the end of an operation. */
  autosaveDue = false;

  /** Called after an operation when an autosave is due. */
  onIdle: () => void = () => undefined;

  /** True while an operation runs. */
  get busy(): boolean {
    return this.current !== undefined;
  }

  /** Runs `run` as the only operation; others get `busy`. */
  async exclusive<T>(name: string, run: () => Promise<T>): Promise<T> {
    if (this.current !== undefined) fail("busy");
    this.current = name;
    try {
      return await run();
    } finally {
      this.current = undefined;
      if (this.autosaveDue) setImmediate(this.onIdle);
    }
  }
}
