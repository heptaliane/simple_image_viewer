package main

import (
	"gopkg.in/yaml.v3"
)

const SKIP_STEP int = 10

type ViewerActions = string

const (
	ModeInvert      ViewerActions = "mode.invert"
	ModeRotateRight ViewerActions = "mode.rotate.right"
	ModeRotateLeft  ViewerActions = "mode.rotate.left"
	ModeReset       ViewerActions = "mode.reset"

	MoveNext          ViewerActions = "move.next"
	MovePrev          ViewerActions = "move.prev"
	MoveTop           ViewerActions = "move.top"
	MoveLast          ViewerActions = "move.last"
	MoveNextDirectory ViewerActions = "move.directory.next"
	MovePrevDirectory ViewerActions = "move.directory.prev"
	MoveSkipNext      ViewerActions = "move.skip.next"
	MoveSkipPrev      ViewerActions = "move.skip.prev"

	ImageZoomIn  ViewerActions = "image.zoom.in"
	ImageZoomOut ViewerActions = "image.zoom.out"

	Quit ViewerActions = "quit"
)

var DEFAULT_KEY_MAPPING = map[ViewerActions][]string{
	ModeInvert:      {"N"},
	ModeRotateRight: {"R"},
	ModeRotateLeft:  {"L"},
	ModeReset:       {"C"},

	MoveNext:          {"L", "Left", "Space", "Enter"},
	MovePrev:          {"H", "Right"},
	MoveTop:           {"W"},
	MoveLast:          {"E"},
	MoveNextDirectory: {"J", "Up"},
	MovePrevDirectory: {"K", "Down"},
	MoveSkipNext:      {},
	MoveSkipPrev:      {},

	ImageZoomIn:  {"I"},
	ImageZoomOut: {"O"},

	Quit: {"Q", "Escape"},
}

type KeyMapping struct {
	mode struct {
		invert []string
		reset  []string
		rotate struct {
			right []string
			left  []string
		}
	}

	move struct {
		next      []string
		prev      []string
		top       []string
		last      []string
		directory struct {
			next []string
			prev []string
		}
		skip struct {
			next []string
			prev []string
		}
	}

	image struct {
		zoom struct {
			in  []string
			out []string
		}
	}

	quit []string
}

type KeyActionParser struct {
	lut map[string]ViewerActions
}

func NewActionMapping(config string) (*KeyActionParser, error) {
	mapping := KeyMapping{}

	err := yaml.Unmarshal([]byte(config), &mapping)
	if err != nil {
		return nil, err
	}
	lut := make(map[string]ViewerActions)

	insertLut := func(action ViewerActions, values []string) {
		if len(values) == 0 || values == nil {
			values = DEFAULT_KEY_MAPPING[action]
		}

		for _, key := range values {
			lut[key] = action
		}
	}

	insertLut(ModeInvert, mapping.mode.invert)
	insertLut(ModeReset, mapping.mode.reset)
	insertLut(ModeRotateLeft, mapping.mode.rotate.left)
	insertLut(ModeRotateRight, mapping.mode.rotate.right)
	insertLut(MoveNext, mapping.move.next)
	insertLut(MovePrev, mapping.move.prev)
	insertLut(MoveTop, mapping.move.top)
	insertLut(MoveLast, mapping.move.last)
	insertLut(MoveNextDirectory, mapping.move.directory.next)
	insertLut(MovePrevDirectory, mapping.move.directory.prev)
	insertLut(MoveSkipNext, mapping.move.skip.next)
	insertLut(MoveSkipPrev, mapping.move.skip.prev)
	insertLut(ImageZoomIn, mapping.image.zoom.in)
	insertLut(ImageZoomOut, mapping.image.zoom.out)
	insertLut(Quit, mapping.quit)

	return &KeyActionParser{
		lut: lut,
	}, nil
}

func (obj *KeyActionParser) GetViewerAction(key string) *ViewerActions {
	action, exists := obj.lut[key]
	if exists {
		return &action
	}
	return nil
}
