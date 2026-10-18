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

type EncryptedProtocol = Exclude<Protocol, "udp" | "tcp">;

/** Per-protocol guidance, so the address field is never a guess. */
const HINTS: Record<EncryptedProtocol, { placeholder: string; help: string }> =
  {
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
  { value: "doh" as const, label: "DoH", hint: "DNS over HTTPS" },
  { value: "dot" as const, label: "DoT", hint: "DNS over TLS" },
  { value: "doq" as const, label: "DoQ", hint: "DNS over QUIC" },
  { value: "doh3" as const, label: "DoH3", hint: "DNS over HTTP/3" },
];

const splitList = (value: string) =>
  value
    .split(",")
    .map((part) => part.trim())
    .filter(Boolean);

/**
 * Whether an encrypted address is named by hostname, and so needs IPs to be
 * reached without a lookup.
 *
 * An endpoint addressed by a literal IP has nothing to look up, so demanding
 * IPs there would be asking for the answer to a question nobody is going to
 * ask. This mirrors the backend rule; the backend remains the authority, and
 * this only decides whether the field is marked required.
 */
function needsBootstrap(address: string): boolean {
  const first = address.split(",")[0]?.trim() ?? "";
  if (!first) return false;

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
  const [ips, setIps] = useState("");
  const [protocol, setProtocol] = useState<EncryptedProtocol>("doh");
  const [address, setAddress] = useState("");
  const [tags, setTags] = useState<string[]>([]);

  const add = useAddCustomServer();
  const hasAddress = Boolean(address.trim());
  // Without an encrypted address the IPs are the server itself; with one named
  // by hostname they are how it is reached. Only an address that is already an
  // IP stands on its own.
  const ipsRequired = !hasAddress || needsBootstrap(address);

  function reset() {
    setName("");
    setIps("");
    setAddress("");
    setTags([]);
    setProtocol("doh");
  }

  /**
   * Clearing on close rather than only on success.
   *
   * Cancelling used to leave every field populated, so the next "Add server"
   * opened onto someone else's half-finished entry.
   */
  function handleOpenChange(next: boolean) {
    setOpen(next);
    if (!next) {
      reset();
      add.reset();
    }
  }

  const canSubmit = Boolean(name.trim() && (!ipsRequired || ips.trim()));

  function submit() {
    if (!canSubmit || add.isPending) return;

    // No encrypted address means plain DNS straight to the IPs. With one, the
    // same IPs become its bootstrap, so they are never typed twice.
    const server = hasAddress
      ? {
          protocol,
          addrs: splitList(address),
          bootstrap: splitList(ips),
        }
      : { protocol: "udp" as const, addrs: splitList(ips), bootstrap: [] };

    add.mutate(
      { name, ...server, tags },
      {
        onSuccess: (saved) => {
          toast.success(`${saved.name} added`);
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
        <Button
          variant="primary"
          size="sm"
          startIcon={<Plus />}
          className="rounded-full"
        >
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
          nothing — which in a multi-field dialog reads as the dialog being
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

        <Field
          label="IP addresses"
          required={ipsRequired}
          description={
            !hasAddress
              ? "Comma separated. Enough on their own — the server is saved as plain DNS."
              : ipsRequired
                ? "Used to reach the encrypted address without resolving its name — otherwise the first lookup would depend on the resolver being configured."
                : "Not needed: the encrypted address is already an IP, so there is no name to look up."
          }
        >
          <Input
            value={ips}
            onChange={(event) => setIps(event.target.value)}
            placeholder="1.1.1.1, 1.0.0.1"
            className="font-mono"
            spellCheck={false}
            autoComplete="off"
          />
        </Field>

        <Field
          label="Encrypted address"
          description={
            hasAddress
              ? HINTS[protocol].help
              : "Optional. Leave empty to use the IPs above as plain DNS."
          }
        >
          <div className="grid grid-cols-[112px_1fr] gap-2">
            <Select
              label="Protocol"
              value={protocol}
              onValueChange={(value) => setProtocol(value ?? "doh")}
              options={PROTOCOL_OPTIONS}
            />
            <Input
              value={address}
              onChange={(event) => setAddress(event.target.value)}
              placeholder={HINTS[protocol].placeholder}
              className="font-mono"
              spellCheck={false}
              autoComplete="off"
            />
          </div>
        </Field>

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
