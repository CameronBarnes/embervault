-- Catch and prevent cyclical tag relationships
CREATE FUNCTION prevent_tag_cycles()
RETURNS trigger AS $$
BEGIN
	-- Self check here just to be safe, existing CHECK should catch it but it doesnt hurt
	IF NEW.child_id = NEW.parent_id THEN
		RAISE EXCEPTION 'Tag can not be its own parent';
	END IF;

	-- Check if the parent can already reach the child, ie adding a child -> parent relationship
	IF EXISTS (
		WITH RECURSIVE ancestors(id) AS (
			SELECT NEW.parent_id
			UNION
			SELECT tp.parent_id
			FROM tag_partents tp
			JOIN ancestors a ON tp.child_id = a.id
		)
		SELECT 1
		FROM ancestors
		WHERE id = NEW.child_id
	) THEN
		RAISE EXCEPTION 'Cycle detected: adding % -> % would create a loop',
			NEW.child_id, NEW.parent_id;
	END IF;

	RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_cycles
BEFORE INSERT OR UPDATE ON tag_parents
FOR EACH ROW
EXECUTE FUNCTION prevent_tag_cycles();
