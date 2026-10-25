import { describe, expect, it, vi } from "vitest";
import {
  CORE_PORT_TAG,
  describeError,
  isPlatformError,
  onCorePort,
  type PortSource,
} from "./platform.js";

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

describe("error form of S-6", () => {
  it("recognises { code, textKey, params } and describes it for the console", () => {
    const error = {
      code: "limitExceeded",
      textKey: "save.error.limitExceeded",
      params: { limit: "file", max: 5 },
    };
    expect(isPlatformError(error)).toBe(true);
    expect(describeError(error)).toBe("limitExceeded (limit=file, max=5)");
    expect(describeError({ code: "busy", textKey: "platform.error.busy", params: {} })).toBe(
      "busy",
    );
    expect(
      describeError({
        code: "blocked",
        textKey: "command.rejected.blocked",
        params: { object: { key: "object.bench" } },
      }),
    ).toBe("blocked (object=object.bench)");
    expect(isPlatformError({ code: "busy" })).toBe(false);
    expect(describeError(new Error("boom"))).toBe("boom");
    expect(describeError("text")).toBe("text");
  });
});
