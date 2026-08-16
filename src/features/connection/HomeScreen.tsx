import { Lock, Power, ShieldCheck, Unlock } from "lucide-react";
import { useMemo, useState } from "react";

import {
  Badge,
  Button,
  CopyableValue,
  Select,
  Skeleton,
  StatusDot,
  toast,
  type SelectOption,
} from "@/design/primitives";
import { useConnection } from "@/features/connection/useConnection";
import { useServers } from "@/features/servers/useServers";
import { useFavourites } from "@/features/servers/useServers";
import { cn } from "@/lib/cn";
import type { Protocol, Server } from "@/lib/ipc";

export function HomeScreen() {
  const { data: servers, isPending } = useServers();
  const { favourites } = useFavourites();
  const {
    active,
    adapter,
    connect,
    disconnect,
    isConnecting,
    isDisconnecting,
  } = useConnection();

  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [protocol, setProtocol] = useState<Protocol | null>(null);

  // Favourites float to the top of the picker for the same reason they do in
  // the browser: they are the entries this user actually returns to.
  const options: SelectOption<string>[] = useMemo(() => {
    if (!servers) return [];

    const rank = (server: Server) => (favourites.includes(server.id) ? 0 : 1);
    return [...servers]
      .sort((a, b) => rank(a) - rank(b))
      .map((server) => ({
        value: server.id,
        label: server.displayName,
        hint: server.endpoints.map((e) => e.label).join(" · "),
      }));
  }, [servers, favourites]);

  const selected = servers?.find((s) => s.id === selectedId) ?? null;

  const protocolOptions: SelectOption<string>[] = useMemo(
    () =>
      selected?.endpoints.map((endpoint) => ({
        value: endpoint.protocol,
        label: endpoint.label,
        hint: endpoint.encrypted ? "Encrypted" : "Unencrypted",
      })) ?? [],
    [selected],
  );

  const chosenProtocol = protocol ?? selected?.protocols[0] ?? null;
  const endpoint = selected?.endpoints.find(
    (e) => e.protocol === chosenProtocol,
  );

  const connected = active !== null;

  function handleConnect() {
    if (!selected || !chosenProtocol) return;

    connect(
      { server: selected, protocol: chosenProtocol },
      {
        onSuccess: () => toast.success(`${selected.displayName} connected`),
        onError: (err: unknown) =>
          toast.error("Could not connect", {
            description: err instanceof Error ? err.message : String(err),
          }),
      },
    );
  }

  function handleDisconnect() {
    disconnect(undefined, {
      onSuccess: () => toast.success("Disconnected"),
      onError: (err: unknown) =>
        toast.error("Could not disconnect", {
          description: err instanceof Error ? err.message : String(err),
        }),
    });
  }

  return (
    <div className="mx-auto flex h-full max-w-lg flex-col justify-center gap-6 p-6">
      <div className="text-center">
        <h1 className="text-2xl font-semibold tracking-tight">ForceConnect</h1>
        <p className="text-text-muted mt-1 text-sm">
          {adapter
            ? `Applies to ${adapter.name}`
            : "Looking for a network adapter…"}
        </p>
      </div>

      {/* The status ring is the one place a glow is used — it is the only
          element that represents a live, ongoing state. */}
      <div className="flex justify-center">
        <div
          className={cn(
            "flex size-36 flex-col items-center justify-center rounded-full border-2 transition-all duration-500",
            connected
              ? "border-accent bg-accent-surface text-accent shadow-[0_0_40px_-8px_var(--accent-glow)]"
              : "border-border-default bg-surface-subtle text-text-faint",
          )}
        >
          {connected ? (
            <ShieldCheck className="size-9" />
          ) : (
            <Power className="size-9" />
          )}
          <span className="mt-2 text-xs font-medium tracking-wide uppercase">
            {connected ? "Protected" : "Off"}
          </span>
        </div>
      </div>

      <div className="flex flex-col gap-3">
        {isPending ? (
          <Skeleton className="h-10 w-full rounded-xl" />
        ) : (
          <Select
            label="DNS server"
            value={selectedId}
            onValueChange={(value) => {
              setSelectedId(value);
              // The previous protocol may not exist on the new server.
              setProtocol(null);
            }}
            options={options}
            placeholder="Choose a DNS server…"
            disabled={connected || isConnecting}
            className="h-10 rounded-xl"
          />
        )}

        {selected && protocolOptions.length > 1 ? (
          <Select
            label="Protocol"
            value={chosenProtocol}
            onValueChange={(value) => setProtocol(value as Protocol | null)}
            options={protocolOptions}
            disabled={connected || isConnecting}
            className="h-10 rounded-xl"
          />
        ) : null}

        {endpoint ? (
          <div className="border-border-subtle bg-surface flex flex-wrap items-center gap-2 rounded-xl border px-3 py-2.5">
            <Badge tone={endpoint.encrypted ? "encrypted" : "neutral"} mono>
              {endpoint.encrypted ? (
                <Lock className="size-2.5" />
              ) : (
                <Unlock className="size-2.5" />
              )}
              {endpoint.label}
            </Badge>
            {endpoint.addrs.map((addr) => (
              <CopyableValue key={addr} value={addr} />
            ))}
          </div>
        ) : null}

        {connected ? (
          <Button
            variant="secondary"
            size="lg"
            block
            onClick={handleDisconnect}
            disabled={isDisconnecting}
          >
            {isDisconnecting ? "Disconnecting…" : "Disconnect"}
          </Button>
        ) : (
          <Button
            variant="primary"
            size="lg"
            block
            onClick={handleConnect}
            disabled={!selected || isConnecting}
          >
            {isConnecting ? "Connecting…" : "Connect"}
          </Button>
        )}
      </div>

      {active ? (
        <div className="text-text-muted flex items-center justify-center gap-2 text-xs">
          <StatusDot tone="success" pulse />
          {active.serverName} · {active.protocol.toUpperCase()} ·{" "}
          {active.adapterName}
        </div>
      ) : null}
    </div>
  );
}
