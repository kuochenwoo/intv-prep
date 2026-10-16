type UpdateGuard = () => void | Promise<void>;
const guards = new Set<UpdateGuard>();

/** Mounted practice pages flush their real save queues before app replacement. */
export function registerUpdateGuard(guard: UpdateGuard) {
  guards.add(guard);
  return () => {
    guards.delete(guard);
  };
}
export async function prepareFrontendUpdate() {
  for (const guard of guards) await guard();
}
