import { describe, expect, it } from "vitest";
import { OperationLock } from "./operation-lock.js";

describe("OperationLock", () => {
  it("runs one operation at a time and refuses the second with busy", async () => {
    const lock = new OperationLock();
    let release: () => void = () => undefined;
    const first = lock.exclusive(
      "save",
      () =>
        new Promise<string>((resolve) => {
          release = () => {
            resolve("done");
          };
        }),
    );
    expect(lock.busy).toBe(true);
    await expect(lock.exclusive("load", () => Promise.resolve(1))).rejects.toMatchObject({
      reason: { code: "busy" },
    });
    release();
    await expect(first).resolves.toBe("done");
    expect(lock.busy).toBe(false);
    await expect(lock.exclusive("load", () => Promise.resolve(1))).resolves.toBe(1);
  });

  it("frees the lock after a failure", async () => {
    const lock = new OperationLock();
    await expect(lock.exclusive("a", () => Promise.reject(new Error("x")))).rejects.toThrow("x");
    expect(lock.busy).toBe(false);
  });

  it("calls onIdle after an operation only while an autosave is due", async () => {
    const lock = new OperationLock();
    let calls = 0;
    lock.onIdle = () => {
      calls += 1;
    };
    await lock.exclusive("a", () => Promise.resolve());
    await new Promise((resolve) => setImmediate(resolve));
    expect(calls).toBe(0);
    lock.autosaveDue = true;
    await lock.exclusive("b", () => Promise.resolve());
    await new Promise((resolve) => setImmediate(resolve));
    expect(calls).toBe(1);
  });
});
