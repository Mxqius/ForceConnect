import { Plus } from "lucide-react";
import { useState } from "react";

import {
  Button,
  ChipInput,
  Dialog,
  Field,
  Input,
  Select,
  toast,
} from "@/design/primitives";
import { useAddCustomServer } from "@/features/servers/useServers";
import { IpcError, type Protocol } from "@/lib/ipc";

/** Per-protocol guidance, so the address field is never a guess. */
const HINTS: Record<Protocol, { placeholder: string; help: string }> = {
  udp: {
    placeholder: "1.1.1.1, 1.0.0.1",
    help: "IP addresses, comma separated. Hostnames cannot be used here — resolving one would need the DNS you are about to configure.",
  },
  tcp: {
    placeholder: "1.1.1.1, 1.0.0.1",
    help: "IP addresses, comma separated. Hostnames cannot be used here — resolving one would need the DNS you are about to configure.",
  },
  dot: {
    placeholder: "dns.example.com",
    help: "The hostname, optionally with a port. It is checked against the server's certificate, so it has to be the name the provider published.",
  },
  doh: {
    placeholder: "dns.example.com",
    help: "A hostname is enough — /dns-query is assumed. Paste the full https:// URL instead if your provider serves a different path.",
  },
  doq: {
    placeholder: "dns.example.com",
    help: "The hostname, optionally with a port. It is checked against the server's certificate, so it has to be the name the provider published.",
  },
  doh3: {
    placeholder: "dns.example.com",
    help: "A hostname is enough — /dns-query is assumed. Paste the full https:// URL instead if your provider serves a different path.",
  },
};

const PROTOCOL_OPTIONS = [
  { value: "udp" as const, label: "DNS", hint: "Unencrypted, UDP/53" },
  { value: "dot" as const, label: "DoT", hint: "DNS over TLS" },
  { value: "doh" as const, label: "DoH", hint: "DNS over HTTPS" },
  { value: "doq" as const, label: "DoQ", hint: "DNS over QUIC" },
  { value: "doh3" as const, label: "DoH3", hint: "DNS over HTTP/3" },
];

/**
 * Whether an encrypted endpoint still needs bootstrap IPs.
 *
 * It only needs them when there is a name to resolve. An endpoint addressed by
 * a literal IP has nothing to look up, so demanding bootstrap there would be
 * asking for the answer to a question nobody is going to ask. This mirrors the
 * backend rule; the backend remains the authority, and this only decides
 * whether the field is marked required.
 */
function needsBootstrap(protocol: Protocol, addrs: string): boolean {
  if (protocol === "udp" || protocol === "tcp") return false;

  const first = addrs.split(",")[0]?.trim() ?? "";
  if (!first) return true;

  const host = first
    .replace(/^https?:\/\//, "")
    .split("/")[0]!
    .replace(/^\[(.+)\]$/, "$1")
    .replace(/:\d+$/, "");

  const isIpv4 = /^\d{1,3}(\.\d{1,3}){3}$/.test(host);
  const isIpv6 = host.includes(":");
  return !isIpv4 && !isIpv6;
}

export function AddServerDialog() {
  const [open, setOpen] = useState(false);
  const [name, setName] = useState("");
  const [protocol, setProtocol] = useState<Protocol>("doh");
  const [addrs, setAddrs] = useState("");
  const [bootstrap, setBootstrap] = useState("");
  const [tags, setTags] = useState<string[]>([]);

  const add = useAddCustomServer();
  const encrypted = protocol !== "udp" && protocol !== "tcp";
  const bootstrapRequired = needsBootstrap(protocol, addrs);

  function reset() {
    setName("");
    setAddrs("");
    setBootstrap("");
    setTags([]);
    setProtocol("doh");
  }

  /**
   * Clearing on close rather than only on success.
   *
   * Cancelling used to leave every field populated, so the next "Add server"
   * opened onto someone else's half-finished entry — and the stale bootstrap
   * IPs were still being submitted even after switching to a protocol whose
   * form does not show that field.
   */
  function handleOpenChange(next: boolean) {
    setOpen(next);
    if (!next) {
      reset();
      add.reset();
    }
  }

  const canSubmit = Boolean(
    name.trim() && addrs.trim() && (!bootstrapRequired || bootstrap.trim()),
  );

  function submit() {
    if (!canSubmit || add.isPending) return;

    add.mutate(
      {
        name,
        protocol,
        addrs: addrs
          .split(",")
          .map((a) => a.trim())
          .filter(Boolean),
        // Plain DNS resolves nothing, so anything left in the field from an
        // earlier protocol choice would only be a validation failure waiting
        // to happen.
        bootstrap: encrypted
          ? bootstrap
              .split(",")
              .map((b) => b.trim())
              .filter(Boolean)
          : [],
        tags,
      },
      {
        onSuccess: (server) => {
          toast.success(`${server.name} added`);
          handleOpenChange(false);
        },
        onError: (error) => {
          // The backend validates through the same rules as the built-in
          // catalog, so its message is the precise one — no need to guess.
          toast.error("Could not add that server", {
            description:
              error instanceof IpcError ? error.message : String(error),
          });
        },
      },
    );
  }

  return (
    <Dialog
      open={open}
      onOpenChange={handleOpenChange}
      trigger={
        <Button variant="primary" size="sm" startIcon={<Plus />}>
          Add server
        </Button>
      }
      title="Add a custom server"
      description="Validated before saving, so it cannot be stored in a state that would fail on connect."
      footer={
        <>
          <Button onClick={() => handleOpenChange(false)}>Cancel</Button>
          <Button
            variant="primary"
            onClick={submit}
            disabled={add.isPending || !canSubmit}
          >
            {add.isPending ? "Saving…" : "Save server"}
          </Button>
        </>
      }
    >
      {/* A real form element, so Enter submits from any field rather than doing
          nothing — which in a five-field dialog reads as the dialog being
          broken. */}
      <form
        className="grid gap-4"
        onSubmit={(event) => {
          event.preventDefault();
          submit();
        }}
      >
        <Field label="Name" required>
          <Input
            value={name}
            onChange={(event) => setName(event.target.value)}
            placeholder="My resolver"
            autoFocus
          />
        </Field>

        <Field label="Protocol">
          <Select
            label="Protocol"
            value={protocol}
            onValueChange={(value) => setProtocol(value ?? "doh")}
            options={PROTOCOL_OPTIONS}
          />
        </Field>

        <Field label="Address" required description={HINTS[protocol].help}>
          <Input
            value={addrs}
            onChange={(event) => setAddrs(event.target.value)}
            placeholder={HINTS[protocol].placeholder}
            className="font-mono"
            spellCheck={false}
            autoComplete="off"
          />
        </Field>

        {encrypted ? (
          <Field
            label="Bootstrap addresses"
            required={bootstrapRequired}
            description={
              bootstrapRequired
                ? "IPs used to reach the server without resolving its name — otherwise the first lookup would depend on the resolver being configured."
                : "Not needed: the address above is already an IP, so there is no name to look up."
            }
          >
            <Input
              value={bootstrap}
              onChange={(event) => setBootstrap(event.target.value)}
              placeholder="1.1.1.1, 1.0.0.1"
              className="font-mono"
              spellCheck={false}
              autoComplete="off"
              disabled={!bootstrapRequired}
            />
          </Field>
        ) : null}

        <Field label="Tags" description="Enter or comma to commit.">
          <ChipInput value={tags} onChange={setTags} max={6} />
        </Field>

        {/* Submits the form on Enter without adding a second visible button;
            the real actions live in the dialog footer. */}
        <button type="submit" className="hidden" aria-hidden tabIndex={-1} />
      </form>
    </Dialog>
  );
}
