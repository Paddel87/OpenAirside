import { describe, expect, it, vi } from "vitest";
import { CORE_PORT_TAG, onCorePort, type PortSource } from "./platform.js";

describe("onCorePort", () => {
  it("passes only tagged ports of the own window", () => {
    let listener: ((event: MessageEvent) => void) | undefined;
    const target: PortSource = {
      addEventListener: (_type, l) => {
        listener = l;
      },
    };
    const self = {};
    const callback = vi.fn();
    onCorePort(target, self, callback);
    const port = {} as MessagePort;
    const send = (data: unknown, source: unknown, ports: MessagePort[]) => {
      listener?.({ data, source, ports } as unknown as MessageEvent);
    };
    send(CORE_PORT_TAG, {}, [port]);
    send("other", self, [port]);
    send(CORE_PORT_TAG, self, []);
    expect(callback).not.toHaveBeenCalled();
    send(CORE_PORT_TAG, self, [port]);
    expect(callback).toHaveBeenCalledWith(port);
  });
});
