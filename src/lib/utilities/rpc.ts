import { createTauRPCProxy } from '$generated/binding';

const rpc = await createTauRPCProxy();

export { rpc };
