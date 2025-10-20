import { ImageGrid } from '@/components/ImageGrid'
import { ImageUpload } from '@/components/ImageUpload'
import { Separator } from '@/components/ui/separator'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/')({
  component: App,
})

function App() {
  return (
    <>
      <main className="p-6 w-full h-full">
        <div className="mx-auto">
          <ImageUpload />
        </div>
        <Separator />
        <div className='flex flex-wrap p-4'>
          <ImageGrid />
        </div>
      </main>
    </>
  )
}
