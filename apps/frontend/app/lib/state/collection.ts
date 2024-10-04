import type { EntityLot } from "@ryot/generated/graphql/backend/graphql";
import { isEqual } from "@ryot/ts-utils";
import { produce } from "immer";
import { atom, useAtom } from "jotai";

type BulkEditingCollectionEntity = { entityId: string; entityLot: EntityLot };

export type BulkEditingCollectionData = {
	collectionId: string;
	entities: Array<BulkEditingCollectionEntity>;
};

const bulkEditingCollectionAtom = atom<BulkEditingCollectionData | null>(null);

export const useBulkEditCollection = () => {
	const [bulkEditingCollection, setBulkEditingCollection] = useAtom(
		bulkEditingCollectionAtom,
	);

	const findIndex = (entity: BulkEditingCollectionEntity) =>
		(bulkEditingCollection?.entities || []).findIndex((f) =>
			isEqual(f, entity),
		);

	const start = (collectionId: string) => {
		setBulkEditingCollection({ collectionId, entities: [] });
	};

	const add = (
		entity: BulkEditingCollectionEntity | Array<BulkEditingCollectionEntity>,
	) => {
		if (!bulkEditingCollection) return;
		if (Array.isArray(entity)) {
			setBulkEditingCollection({ ...bulkEditingCollection, entities: entity });
			return;
		}
		if (findIndex(entity) !== -1) return;
		setBulkEditingCollection(
			produce(bulkEditingCollection, (draft) => {
				draft.entities.push(entity);
			}),
		);
	};

	const remove = (entity: BulkEditingCollectionEntity) => {
		if (!bulkEditingCollection) return;
		setBulkEditingCollection(
			produce(bulkEditingCollection, (draft) => {
				draft.entities.splice(findIndex(entity), 1);
			}),
		);
	};

	const stop = () => setBulkEditingCollection(null);

	return {
		add,
		stop,
		start,
		remove,
		state: bulkEditingCollection
			? {
					size: bulkEditingCollection.entities.length,
					entities: bulkEditingCollection.entities,
					isAdded: (entity: BulkEditingCollectionEntity) =>
						findIndex(entity) !== -1,
				}
			: (false as const),
	};
};
