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
    help: "IP addresses, comma separated.",
  },
  tcp: { placeholder: "1.1.1.1", help: "IP addresses, comma separated." },
  dot: {
    placeholder: "dns.example.com",
    help: "Hostname — it is checked against the server's certificate.",
  },
  doh: {
    placeholder: "https://dns.example.com/dns-query",
    help: "Full HTTPS URL including the path.",
  },
  doq: {
    placeholder: "dns.example.com",
    help: "Hostname — it is checked against the server's certificate.",
  },
  doh3: {
    placeholder: "https://dns.example.com/dns-query",
    help: "Full HTTPS URL including the path.",
  },
};

const PROTOCOL_OPTIONS = [
  { value: "udp" as const, label: "DNS", hint: "Unencrypted, UDP/53" },
  { value: "dot" as const, label: "DoT", hint: "DNS over TLS" },
  { value: "doh" as const, label: "DoH", hint: "DNS over HTTPS" },
  { value: "doq" as const, label: "DoQ", hint: "DNS over QUIC" },
  { value: "doh3" as const, label: "DoH3", hint: "DNS over HTTP/3" },
];

export function AddServerDialog() {
  const [open, setOpen] = useState(false);
  const [name, setName] = useState("");
  const [protocol, setProtocol] = useState<Protocol>("doh");
  const [addrs, setAddrs] = useState("");
  const [bootstrap, setBootstrap] = useState("");
  const [tags, setTags] = useState<string[]>([]);

  const add = useAddCustomServer();
  const encrypted = protocol !== "udp" && protocol !== "tcp";

  function reset() {
    setName("");
    setAddrs("");
    setBootstrap("");
    setTags([]);
    setProtocol("doh");
  }

  function submit() {
    add.mutate(
      {
        name,
        protocol,
        addrs: addrs.split(",").map((a) => a.trim()),
        bootstrap: bootstrap.split(",").map((b) => b.trim()),
        tags,
      },
      {
        onSuccess: (server) => {
          toast.success(`${server.name} added`);
          setOpen(false);
          reset();
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
      onOpenChange={setOpen}
      trigger={
        <Button variant="primary" size="sm" startIcon={<Plus />}>
          Add server
        </Button>
      }
      title="Add a custom server"
      description="Validated before saving, so it cannot be stored in a state that would fail on connect."
      footer={
        <>
          <Button onClick={() => setOpen(false)}>Cancel</Button>
          <Button
            variant="primary"
            onClick={submit}
            disabled={add.isPending || !name.trim() || !addrs.trim()}
          >
            {add.isPending ? "Saving…" : "Save server"}
          </Button>
        </>
      }
    >
      <div className="grid gap-4">
        <Field label="Name" required>
          <Input
            value={name}
            onChange={(event) => setName(event.target.value)}
            placeholder="My resolver"
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
          />
        </Field>

        {encrypted ? (
          <Field
            label="Bootstrap addresses"
            required
            description="IPs used to reach the server without resolving its name — otherwise the first lookup would depend on the resolver being configured."
          >
            <Input
              value={bootstrap}
              onChange={(event) => setBootstrap(event.target.value)}
              placeholder="1.1.1.1, 1.0.0.1"
              className="font-mono"
            />
          </Field>
        ) : null}

        <Field label="Tags" description="Enter or comma to commit.">
          <ChipInput value={tags} onChange={setTags} max={6} />
        </Field>
      </div>
    </Dialog>
  );
}
