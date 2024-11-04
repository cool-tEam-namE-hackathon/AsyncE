import { ref } from "vue";

async function useBackendImmediately(
    action: () => void,
    onSucess?: () => void,
    onFailure?: () => void,
) {
    const isLoading = ref<boolean>(true);
    const errorMessage = ref<string | null>(null);

    try {
        await action();
        if (onSucess) onSucess();
    } catch (e) {
        const error = e as Error;
        errorMessage.value = error.message;
        if (onFailure) onFailure();
    } finally {
        isLoading.value = false;
    }

    return [isLoading, errorMessage];
}

export { useBackendImmediately };
