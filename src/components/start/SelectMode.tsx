import { type sizeT, options } from '../../types'

export function SelectMode({ height, setIndex }: { height: sizeT['height']; setIndex: (i: number) => void }) {
  return (
    <box width={50} justifyContent="center" alignItems="center">
      <select
        marginTop={1}
        paddingTop={1}
        height={height > 13 ? 8 : 2}
        style={{
          width: 20,
          selectedTextColor: 'white',
          focusedTextColor: '#333333',
          selectedBackgroundColor: '#352F99',
        }}
        options={options}
        focused={true}
        onChange={(index, option) => {
          setIndex(index)
        }}
      />
    </box>
  )
}
