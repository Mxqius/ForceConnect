import { Globe, RefreshCw, Search, Trash2, Wifi } from "lucide-react";
import { useState, type ReactNode } from "react";

import {
  AlertDialog,
  Badge,
  Button,
  Checkbox,
  Chip,
  ChipInput,
  CopyableValue,
  Dialog,
  EmptyState,
  Field,
  IconButton,
  Input,
  LatencyBadge,
  Popover,
  Progress,
  Select,
  Separator,
  Skeleton,
  StatusDot,
  Switch,
  Tab,
  TabList,
  TabPanel,
  Tabs,
  Tooltip,
  toast,
  type BadgeTone,
  type ButtonVariant,
} from "@/design/primitives";
import { cn } from "@/lib/cn";

/**
 * Every primitive in every state, on one page.
 *
 * This exists so a design change can be reviewed in one pass instead of by
 * clicking through the app hunting for the affected component. It is reachable
 * at `/dev/kitchen-sink` in development builds.
 */
export function KitchenSink() {
  const [theme, setTheme] = useState<"dark" | "light">("dark");

  function toggleTheme() {
    const next = theme === "dark" ? "light" : "dark";
    setTheme(next);
    document.documentElement.dataset.theme = next;
  }

  return (
    <div className="fc-scroll h-full overflow-y-auto">
      <div className="mx-auto flex max-w-4xl flex-col gap-10 p-8">
        <header className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-2xl font-semibold tracking-tight">
              Design system
            </h1>
            <p className="text-text-muted mt-1 text-sm">
              Every primitive, every state. Toggle the theme to check both
              palettes.
            </p>
          </div>
          <Button onClick={toggleTheme} variant="secondary">
            {theme === "dark" ? "Light theme" : "Dark theme"}
          </Button>
        </header>

        <Section title="Buttons">
          <Row label="Variants">
            {(["primary", "secondary", "ghost", "danger", "link"] as const).map(
              (variant: ButtonVariant) => (
                <Button key={variant} variant={variant}>
                  {variant}
                </Button>
              ),
            )}
          </Row>
          <Row label="Sizes">
            <Button size="sm">Small</Button>
            <Button size="md">Medium</Button>
            <Button size="lg">Large</Button>
          </Row>
          <Row label="With icons">
            <Button variant="primary" startIcon={<Wifi />}>
              Connect
            </Button>
            <Button startIcon={<RefreshCw />}>Refresh</Button>
            <Button variant="danger" startIcon={<Trash2 />}>
              Delete
            </Button>
          </Row>
          <Row label="Disabled">
            <Button variant="primary" disabled>
              Primary
            </Button>
            <Button disabled>Secondary</Button>
            <Button variant="ghost" disabled>
              Ghost
            </Button>
          </Row>
          <Row label="Icon buttons">
            <IconButton label="Refresh">
              <RefreshCw />
            </IconButton>
            <IconButton label="Delete" variant="danger">
              <Trash2 />
            </IconButton>
            <IconButton label="Disabled" disabled>
              <Search />
            </IconButton>
          </Row>
        </Section>

        <Section title="Badges and status">
          <Row label="Tones">
            {(
              [
                "neutral",
                "accent",
                "success",
                "warning",
                "danger",
                "encrypted",
              ] as const
            ).map((tone: BadgeTone) => (
              <Badge key={tone} tone={tone}>
                {tone}
              </Badge>
            ))}
          </Row>
          <Row label="Protocols">
            <Badge tone="neutral" mono>
              DNS
            </Badge>
            <Badge tone="encrypted" mono>
              DoH
            </Badge>
            <Badge tone="encrypted" mono>
              DoT
            </Badge>
            <Badge tone="encrypted" mono>
              DoQ
            </Badge>
            <Badge tone="encrypted" mono>
              DoH3
            </Badge>
          </Row>
          <Row label="Latency">
            <LatencyBadge ms={18} />
            <LatencyBadge ms={120} />
            <LatencyBadge ms={155} />
            <LatencyBadge ms={340} />
            <LatencyBadge ms={0.4} />
            <LatencyBadge ms={null} />
            <LatencyBadge />
            <LatencyBadge pending />
          </Row>
          <Row label="Status dots">
            <span className="flex items-center gap-1.5 text-sm">
              <StatusDot tone="success" pulse /> Connected
            </span>
            <span className="flex items-center gap-1.5 text-sm">
              <StatusDot tone="warning" /> Degraded
            </span>
            <span className="flex items-center gap-1.5 text-sm">
              <StatusDot tone="danger" /> Failed
            </span>
            <span className="flex items-center gap-1.5 text-sm">
              <StatusDot tone="neutral" /> Idle
            </span>
          </Row>
          <Row label="Chips">
            <Chip>no-logging</Chip>
            <Chip onRemove={() => undefined} removeLabel="Remove tag fast">
              fast
            </Chip>
          </Row>
          <Row label="Copyable">
            <CopyableValue value="1.1.1.1" />
            <CopyableValue value="https://cloudflare-dns.com/dns-query" />
          </Row>
        </Section>

        <Section title="Forms">
          <div className="grid max-w-md gap-4">
            <Field label="Server name" required>
              <Input placeholder="Cloudflare" />
            </Field>
            <Field
              label="Address"
              description="IPv4 or IPv6, optionally with a port."
            >
              <Input placeholder="1.1.1.1" className="font-mono" />
            </Field>
            <Field label="Disabled">
              <Input placeholder="Not editable" disabled />
            </Field>
            <Field label="Tags" description="Enter or comma to commit.">
              <TagDemo />
            </Field>
            <Field label="Protocol">
              <SelectDemo />
            </Field>
          </div>

          <Row label="Toggles">
            <ToggleDemo />
          </Row>
        </Section>

        <Section title="Navigation">
          <Tabs defaultValue="all">
            <TabList>
              <Tab value="all" badge={22}>
                All
              </Tab>
              <Tab value="dns" badge={17}>
                DNS
              </Tab>
              <Tab value="doh" badge={16}>
                DoH
              </Tab>
              <Tab value="dot" badge={14}>
                DoT
              </Tab>
              <Tab value="doq" badge={4}>
                DoQ
              </Tab>
            </TabList>
            <TabPanel value="all" className="text-text-muted pt-4 text-sm">
              All servers.
            </TabPanel>
            <TabPanel value="dns" className="text-text-muted pt-4 text-sm">
              Plain DNS only.
            </TabPanel>
            <TabPanel value="doh" className="text-text-muted pt-4 text-sm">
              DNS over HTTPS.
            </TabPanel>
            <TabPanel value="dot" className="text-text-muted pt-4 text-sm">
              DNS over TLS.
            </TabPanel>
            <TabPanel value="doq" className="text-text-muted pt-4 text-sm">
              DNS over QUIC.
            </TabPanel>
          </Tabs>
        </Section>

        <Section title="Overlays">
          <Row label="Transient">
            <Tooltip content="Re-probe every server">
              <Button startIcon={<RefreshCw />}>Hover me</Button>
            </Tooltip>
            <Popover trigger={<Button>Popover</Button>}>
              <p className="text-text-secondary text-sm">
                Anchored content with a border and shadow.
              </p>
            </Popover>
          </Row>
          <Row label="Modal">
            <DialogDemo />
            <AlertDialogDemo />
          </Row>
          <Row label="Toasts">
            <Button onClick={() => toast.success("Cloudflare connected")}>
              Success
            </Button>
            <Button
              onClick={() =>
                toast.error("Could not bind port 53", {
                  description:
                    "Internet Connection Sharing is using it. Stop the SharedAccess service and try again.",
                })
              }
            >
              Error
            </Button>
            <Button
              onClick={() =>
                toast.warning("DNS changed outside ForceConnect", {
                  description: "Wi-Fi is no longer pointing at the resolver.",
                })
              }
            >
              Warning
            </Button>
            <Button onClick={() => toast.info("Probing 22 servers…")}>
              Info
            </Button>
          </Row>
        </Section>

        <Section title="Feedback">
          <Row label="Progress">
            <div className="w-64">
              <Progress label="Connecting" value={50} />
            </div>
            <div className="w-64">
              <Progress label="Complete" value={100} tone="success" />
            </div>
          </Row>
          <Row label="Skeleton">
            <div className="flex w-64 flex-col gap-2">
              <Skeleton className="h-4 w-1/2" />
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-2/3" />
            </div>
          </Row>
          <div className="border-border-subtle rounded-lg border">
            <EmptyState
              icon={<Globe />}
              title="No servers match"
              description="Try a different protocol filter, or clear the search."
              action={<Button size="sm">Clear filters</Button>}
            />
          </div>
        </Section>

        <Section title="Typography">
          <div className="flex flex-col gap-2">
            <p className="text-2xl font-semibold tracking-tight">
              2xl semibold — screen titles
            </p>
            <p className="text-xl font-semibold">
              xl semibold — section titles
            </p>
            <p className="text-lg font-medium">lg medium — dialog titles</p>
            <p className="text-base">base — body copy</p>
            <p className="text-sm">sm — the app&rsquo;s workhorse size</p>
            <p className="text-text-muted text-xs">xs — hints and metadata</p>
            <p className="text-text-faint text-2xs">2xs — badges</p>
            <p className="font-mono text-sm tabular-nums">
              mono tabular — 192.168.234.241 · 2606:4700:4700::1111 · 18 ms
            </p>
          </div>
        </Section>

        <Section title="Surfaces">
          <div className="grid grid-cols-3 gap-3">
            {(
              [
                ["surface-base", "bg-surface-base"],
                ["surface-subtle", "bg-surface-subtle"],
                ["surface", "bg-surface"],
                ["surface-elevated", "bg-surface-elevated"],
                ["surface-inset", "bg-surface-inset"],
                ["surface-hover", "bg-surface-hover"],
              ] as const
            ).map(([name, klass]) => (
              <div
                key={name}
                className={cn(
                  "border-border-subtle rounded-md border p-3 font-mono text-xs",
                  klass,
                )}
              >
                {name}
              </div>
            ))}
          </div>
          <Separator />
          <p className="text-text-muted text-xs">
            Separator above. Borders separate; shadows are reserved for content
            that genuinely floats.
          </p>
        </Section>
      </div>
    </div>
  );
}

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="flex flex-col gap-4">
      <h2 className="text-text-secondary border-border-subtle border-b pb-2 text-xs font-semibold tracking-widest uppercase">
        {title}
      </h2>
      {children}
    </section>
  );
}

function Row({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="flex items-start gap-4">
      <span className="text-text-faint w-24 shrink-0 pt-1.5 text-xs">
        {label}
      </span>
      <div className="flex flex-wrap items-center gap-2">{children}</div>
    </div>
  );
}

function TagDemo() {
  const [tags, setTags] = useState(["no-logging", "fast"]);
  return <ChipInput value={tags} onChange={setTags} max={6} />;
}

function SelectDemo() {
  const [value, setValue] = useState<string | null>("doh");
  return (
    <Select
      label="Protocol"
      value={value}
      onValueChange={setValue}
      options={[
        { value: "udp", label: "DNS", hint: "Unencrypted, UDP/53" },
        { value: "dot", label: "DoT", hint: "DNS over TLS, 853" },
        { value: "doh", label: "DoH", hint: "DNS over HTTPS, 443" },
        { value: "doq", label: "DoQ", hint: "DNS over QUIC, 853" },
        {
          value: "doh3",
          label: "DoH3",
          hint: "Not offered here",
          disabled: true,
        },
      ]}
    />
  );
}

function ToggleDemo() {
  const [on, setOn] = useState(true);
  const [checked, setChecked] = useState(true);

  return (
    <>
      <label className="flex items-center gap-2 text-sm">
        <Switch checked={on} onCheckedChange={setOn} /> Start on boot
      </label>
      <Switch checked={false} disabled />
      <label className="flex items-center gap-2 text-sm">
        <Checkbox checked={checked} onCheckedChange={setChecked} /> Log queries
      </label>
      <Checkbox indeterminate />
      <Checkbox checked={false} disabled />
    </>
  );
}

function DialogDemo() {
  const [open, setOpen] = useState(false);

  return (
    <Dialog
      open={open}
      onOpenChange={setOpen}
      trigger={<Button>Dialog</Button>}
      title="Add a custom server"
      description="Validated against the selected protocol before it is saved."
      footer={
        <>
          <Button onClick={() => setOpen(false)}>Cancel</Button>
          <Button variant="primary" onClick={() => setOpen(false)}>
            Save server
          </Button>
        </>
      }
    >
      <div className="grid gap-4">
        <Field label="Name" required>
          <Input placeholder="My resolver" />
        </Field>
        <Field label="Address" required>
          <Input placeholder="9.9.9.9" className="font-mono" />
        </Field>
      </div>
    </Dialog>
  );
}

function AlertDialogDemo() {
  const [open, setOpen] = useState(false);

  return (
    <AlertDialog
      open={open}
      onOpenChange={setOpen}
      trigger={<Button variant="danger">Alert dialog</Button>}
      title="Reset DNS settings?"
      description="Every adapter returns to DHCP-assigned DNS. Any active connection is dropped."
      footer={
        <>
          <Button onClick={() => setOpen(false)}>Cancel</Button>
          <Button variant="danger" onClick={() => setOpen(false)}>
            Reset
          </Button>
        </>
      }
    />
  );
}
