-- Your SQL goes here

-- pub struct CardTaskItem {
--     pub id: String,
--     pub card_id: String,
--     pub text: String,
--     pub is_complete: bool,
-- }

create table card_task_item
(
    id          uuid primary key,
    card_id     uuid,
    text_       text,
    is_complete bool
);