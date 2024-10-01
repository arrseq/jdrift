export function delete_element(class_id: number) {
    document.body.querySelector(`.class-${class_id}`)?.remove();
}