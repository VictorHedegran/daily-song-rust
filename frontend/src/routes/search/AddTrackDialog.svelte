<script lang="ts">
	import { X } from 'lucide-svelte';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onSubmit: (data: { notes: string; mood: string }) => void;
		trackName?: string;
		artistName?: string;
	}

	let { isOpen, onClose, onSubmit, trackName, artistName }: Props = $props();

	let notes = $state('');
	let mood = $state('');
	let isSubmitting = $state(false);
	let error = $state<string | null>(null);
	let dialogRef: HTMLDialogElement | undefined;
	let moodInputRef: HTMLInputElement | undefined;

	$effect(() => {
		if (!dialogRef) return;

		if (isOpen) {
			dialogRef.showModal();
			moodInputRef?.focus();
		} else {
			dialogRef.close();
		}
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		isSubmitting = true;
		error = null;
		try {
			await onSubmit({ notes, mood });
			notes = '';
			mood = '';
		} catch {
			error = 'Failed to add song. Please try again.';
		} finally {
			isSubmitting = false;
		}
	}

	function handleClose() {
		if (!isSubmitting) {
			notes = '';
			mood = '';
			error = null;
			onClose();
		}
	}
</script>

<dialog
	bind:this={dialogRef}
	onclose={handleClose}
	class="m-0 mt-auto w-full max-w-md bg-transparent p-0 backdrop:bg-black/70 backdrop:backdrop-blur-sm sm:m-auto"
>
	<div
		class="w-full rounded-t-2xl bg-background-elevated p-5 pb-[calc(1.25rem+env(safe-area-inset-bottom))] shadow-2xl sm:rounded-2xl sm:p-6"
	>
		<div class="mb-3 flex justify-center sm:hidden">
			<div class="h-1 w-10 rounded-full bg-background-highlight"></div>
		</div>

		<div class="mb-4 flex items-center justify-between">
			<div class="min-w-0 flex-1">
				<h2 class="text-xl font-bold text-white">Add Your Daily Song</h2>
				{#if trackName}
					<p class="mt-1 truncate text-sm text-text-secondary">
						{trackName}
						{#if artistName}
							· {artistName}
						{/if}
					</p>
				{/if}
			</div>
			<button
				onclick={handleClose}
				disabled={isSubmitting}
				class="-mr-2 p-2 text-text-secondary transition-colors active:text-white disabled:opacity-50"
				aria-label="Close"
			>
				<X size={24} />
			</button>
		</div>

		{#if error}
			<div
				class="mb-4 rounded-md border border-error bg-error/10 px-3 py-2 text-sm text-error"
				role="alert"
			>
				{error}
			</div>
		{/if}

		<form onsubmit={handleSubmit} class="space-y-4">
			<div>
				<label for="mood" class="mb-2 block text-sm font-semibold text-white">
					How are you feeling?
				</label>
				<input
					bind:this={moodInputRef}
					id="mood"
					type="text"
					bind:value={mood}
					placeholder="Happy, Nostalgic, Energetic..."
					class="w-full rounded-lg border-none bg-input px-4 py-3 text-white transition-shadow outline-none placeholder:text-text-subdued focus:ring-2 focus:ring-spotify-green"
					disabled={isSubmitting}
				/>
			</div>

			<div>
				<label for="notes" class="mb-2 block text-sm font-semibold text-white">
					Notes <span class="font-normal text-text-subdued">(optional)</span>
				</label>
				<textarea
					id="notes"
					bind:value={notes}
					placeholder="Why this song today?"
					rows={3}
					class="w-full resize-none rounded-lg border-none bg-input px-4 py-3 text-white transition-shadow outline-none placeholder:text-text-subdued focus:ring-2 focus:ring-spotify-green"
					disabled={isSubmitting}
				></textarea>
			</div>

			<div class="flex gap-3 pt-2">
				<button
					type="button"
					onclick={handleClose}
					disabled={isSubmitting}
					class="flex-1 rounded-full border border-border-muted px-4 py-3 font-semibold text-white transition-colors active:border-white active:bg-background-highlight disabled:opacity-50"
				>
					Cancel
				</button>
				<button
					type="submit"
					disabled={isSubmitting}
					class="flex-1 rounded-full bg-spotify-green px-4 py-3 font-bold text-black transition-all active:scale-[0.98] active:bg-spotify-green-hover disabled:opacity-50"
				>
					{isSubmitting ? 'Adding...' : 'Add Song'}
				</button>
			</div>
		</form>
	</div>
</dialog>
