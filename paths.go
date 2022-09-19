package main

import (
	"os"
	"path/filepath"
	"sort"
)

func isDirectory(path string) (bool, error) {
	fileInfo, err := os.Stat(path)
	if err != nil {
		return false, err
	}
	return fileInfo.IsDir(), nil
}

func GetParentDirectory(path string) string {
	return filepath.Dir(path)
}

func GetFileList(dirpath string, include_dir bool) ([]string, error) {
	files, err := filepath.Glob(filepath.Join(dirpath, "*"))
	if err != nil {
		return []string{}, err
	}

	paths := []string{}
	for _, path := range files {
		isdir, err := isDirectory(path)
		if err != nil {
			return nil, err
		}

		if include_dir == isdir {
			paths = append(paths, path)
		} else {
			paths = append(paths, path)
		}
	}

	sort.Strings(files)
	return files, err
}

func getNextDirectory(dirpath string) (*string, error) {
	parent := GetParentDirectory(dirpath)
	if parent == dirpath {
		return nil, nil
	}

	dirs, err := GetFileList(parent, true)
	if err != nil {
		return nil, err
	}

	var i int
	for i = range dirs {
		if dirs[i] == dirpath {
			break
		}
	}

	if i+1 < len(dirs) {
		dirname := dirs[i+1]
		for {
			dirs, err = GetFileList(dirname, true)
			if len(dirs) > 0 {
				return &dirname, nil
			}
			dirname = dirs[0]
		}
	}

	return &parent, nil
}

func getPrevDirectory(dirpath string) (*string, error) {
	dirs, err := GetFileList(dirpath, true)
	if err != nil {
		return nil, err
	}

	if len(dirs) > 0 {
		return &dirs[len(dirs)-1], nil
	}

	for {
		parent := GetParentDirectory(dirpath)
		if parent == dirpath {
			break
		}

		dirs, err = GetFileList(parent, true)
		if err != nil {
			return nil, err
		}

		var i int
		for i = range dirs {
			if dirs[i] == dirpath {
				break
			}
		}

		if i > 0 {
			return &dirs[i-1], nil
		}
		dirpath = parent
	}

	return nil, nil
}

type PathProvider struct {
	parent string
	files  []string
	idx    int
}

func NewPathProvider(filepath string) (PathProvider, error) {
	parent := GetParentDirectory(filepath)
	files, err := GetFileList(parent, false)
	if err != nil {
		return PathProvider{}, nil
	}

	for i := range files {
		if files[i] == filepath {
			return PathProvider{parent: parent, files: files, idx: i}, nil
		}
	}
	return PathProvider{parent: parent, files: files, idx: 0}, nil
}

func (provider *PathProvider) MoveTop() {
	provider.idx = 0
}

func (provider *PathProvider) MoveLast() {
	provider.idx = len(provider.files) - 1
}

func (provider *PathProvider) moveDirectory(get_directory_func func(string) (*string, error)) error {
	for {
		dirname, err := get_directory_func(provider.parent)
		if err != nil || dirname == nil {
			return err
		}
		if *dirname == provider.parent {
			return err
		}

		files, err := GetFileList(*dirname, false)
		if err != nil {
			return err
		}
		if len(files) > 0 {
			provider.parent = *dirname
			provider.files = files
			return nil
		}

		provider.parent = *dirname
	}
}

func (provider *PathProvider) MoveNextDirectory() error {
	return provider.moveDirectory(getNextDirectory)
}

func (provider *PathProvider) MovePrevDirectory() error {
	return provider.moveDirectory(getPrevDirectory)
}

func (provider *PathProvider) Move(n_images int) error {
	provider.idx += n_images

	if provider.idx < 0 {
		err := provider.MovePrevDirectory()
		if err != nil {
			return err
		}
		provider.MoveLast()
	}

	if provider.idx >= len(provider.files) {
		err := provider.MoveNextDirectory()
		if err != nil {
			return err
		}
		provider.MoveTop()
	}

	return nil
}

func (provider *PathProvider) Get() string {
	return provider.files[provider.idx]
}
