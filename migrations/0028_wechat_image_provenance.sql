-- Source images are now served through VictoriaPark's stable mirror and
-- generated cards are labelled explicitly. Keep the original values valid for
-- existing packages while accepting the more precise provenance labels.
ALTER TABLE wechat_packages
    DROP CONSTRAINT IF EXISTS wechat_packages_image_origin_check;
ALTER TABLE wechat_packages
    ADD CONSTRAINT wechat_packages_image_origin_check CHECK (
        image_origin IN ('source','victoriapark','source-mirrored','victoriapark-generated')
    );
