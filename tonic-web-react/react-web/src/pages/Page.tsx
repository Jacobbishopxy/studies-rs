/**
 * @file:	Page.tsx
 * @author:	Jacob Xie
 * @date:	2023/09/01 21:08:17 Friday
 * @brief:
 **/

import {ReactElement, useEffect, useState} from 'react'
import {PageClient} from "../PageServiceClientPb"
import {PageRequest, Block} from "../page_pb"

const pageService = new PageClient("http://localhost:2999", null, null)


const Page = (props: {name: string}) => {

    const [page, setPage] = useState<ReactElement[]>([])

    useEffect(() => {
        const request = new PageRequest()
        request.setName(props.name)

        pageService.getPage(request, null).then((pageReply) => {
            const p = pageReply.getBlocksList().map((block, index) => {
                switch (block.getBlockCase()) {
                    case Block.BlockCase.PARAGRAPHBLOCK:
                        // return <p dangerouslySetInnerHTML={{__html: block.getParagraphblock().getData().getText()}} />
                        return <p>1</p>
                    case Block.BlockCase.HEADERBLOCK:
                        // const HeadingTag = `h${block.getHeaderblock().getData().getLevel()}`
                        // return <HeadingTag dangerouslySetInnerHTML={{__html: block.getHeaderblock().getData().getText()}} />
                        return <p>2</p>

                    case Block.BlockCase.LISTBLOCK:
                        // let ListTag: any
                        // let ItemTag: any
                        // if (block.getListblock().getData().getStyle() === proto.page.ListStyle.ORDERED) {
                        //     ListTag = `ol`
                        //     ItemTag = 'li'
                        // } else {
                        //     ListTag = `ul`
                        //     ItemTag = 'li'
                        // }
                        // return (
                        //     <ListTag>
                        //         {block.getListblock().getData().getItemsList().map((item, index) =>
                        //             <ItemTag dangerouslySetInnerHTML={{__html: item}} />
                        //         )}
                        //     </ListTag>
                        // )
                        return <p>3</p>

                    default:
                        return <p>Block is not supported</p>
                }
            })

            setPage(p)
        })
    }, [props.name])

    return (
        <div>{page}</div>
    )
}


export default Page
