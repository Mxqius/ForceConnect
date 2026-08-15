import { Button } from "@base-ui/react/button";
import { useQuery } from "@tanstack/react-query";

import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

export function HomeScreen() {
  const { data, isPending, error, refetch, isFetching } = useQuery({
    queryKey: queryKeys.appInfo,
    queryFn: ipc.appInfo,
  });

  return (
    <div className="flex h-full flex-col items-start justify-center gap-6 p-10">
      <div>
        <h1 className="text-2xl font-semibold tracking-tight">ForceConnect</h1>
        <p className="text-text-muted mt-1 text-sm">
          Foundation is wired. Design system and features come next.
        </p>
      </div>

      <dl className="grid grid-cols-[auto_1fr] gap-x-6 gap-y-2 font-mono text-sm">
        <dt className="text-text-muted">version</dt>
        <dd data-selectable>{isPending ? "…" : (data?.version ?? "—")}</dd>

        <dt className="text-text-muted">host</dt>
        <dd data-selectable>{isPending ? "…" : (data?.os ?? "—")}</dd>

        <dt className="text-text-muted">elevated</dt>
        <dd data-selectable>
          {isPending ? "…" : data ? String(data.elevated) : "—"}
        </dd>
      </dl>

      {error ? (
        <p className="text-sm text-red-400" role="alert">
          {error instanceof Error ? error.message : String(error)}
        </p>
      ) : null}

      <Button
        onClick={() => void refetch()}
        disabled={isFetching}
        className="border-border-subtle bg-surface-raised hover:border-accent rounded-md border px-3 py-1.5 text-sm transition-colors disabled:opacity-50"
      >
        {isFetching ? "Refreshing…" : "Refresh"}
      </Button>
    </div>
  );
}
