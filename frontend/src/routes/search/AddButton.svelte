<script lang="ts">
	import { Plus, Check, CircleAlert } from 'lucide-svelte';
	import AddTrackDialog from './AddTrackDialog.svelte';

	interface Props {
		uri: string;
		trackName?: string;
		artistName?: string;
	}

	let { uri, trackName, artistName }: Props = $props();

	let isDialogOpen = $state(false);
	let status = $state<'idle' | 'success' | 'error'>('idle');
	let isPending = $state(false);

	async function handleSubmit(data: { notes: string; mood: string }) {
		isDialogOpen = false;
		
		isPending = true;

		try {
			const formData = new FormData();
			formData.append('uri', uri);
			formData.append('notes', data.notes);
			formData.append('mood', data.mood);

			const res = await fetch('?/addTrack', {
				method: 'POST',
				body: formData
			});

			if (!res.ok) {
				status = 'error';
			} else {
				status = 'success';
			}
		} catch {
			status = 'error';
		} finally {
			isPending = false;
		}

		setTimeout(() => {
			status = 'idle';
		}, 2000);
	}
</script>

<button
	class={`rounded-full p-2.5 text-sm font-bold transition-all duration-200 active:scale-90 ${
		status === 'success'
			? 'scale-110 bg-spotify-green/20 text-spotify-green'
			: status === 'error'
				? 'bg-error/20 text-error'
				: 'bg-spotify-green text-black active:bg-spotify-green-hover'
	}`}
	onclick={() => {
		if (status === 'idle') isDialogOpen = true;
	}}
	disabled={isPending}
	aria-label={`Add ${trackName || 'track'}`}
>
	{#if status === 'success'}
		<Check size={20} />
	{:else if status === 'error'}
		<CircleAlert size={20} />
	{:else}
		<Plus size={20} />
	{/if}
</button>
<AddTrackDialog
	isOpen={isDialogOpen}
	onClose={() => {
		isDialogOpen = false;
	}}
	onSubmit={handleSubmit}
	{trackName}
	{artistName}
/>
