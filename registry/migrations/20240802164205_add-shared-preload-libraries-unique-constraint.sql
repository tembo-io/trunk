ALTER TABLE public.shared_preload_libraries
ADD CONSTRAINT unique_name UNIQUE (name);
